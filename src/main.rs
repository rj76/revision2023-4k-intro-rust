// #![feature(lang_items, start, raw_ref_op, stmt_expr_attributes)]
// #![feature(no_core)]
// #![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;
use core::{mem, ptr};

// use libc_print::std_name::println;

mod bindings;

#[macro_use]
mod gl_wrapper;

use crate::bindings::{Xlib, glx, gl};

use ::core::ffi::{c_int, c_long, c_ulong, c_uint};
use core::ffi::c_char;

pub type Bool = c_int;
pub const FALSE: Bool = 0;
pub const CWBACK_PIXEL: c_ulong = 0x0002;
pub const KEY_PRESS_MASK: c_long = 0x0000_0001;
pub const KEY_RELEASE_MASK: c_long = 0x0000_0002;
pub const CLIENT_MESSAGE: c_int = 33;
pub const KEY_PRESS: c_int = 2;
pub const LINE_SOLID: c_int = 0;
pub const CAP_BUTT: c_int = 1;
pub const JOIN_BEVEL: c_int = 2;
pub const FILL_SOLID: c_int = 0;
pub const INPUT_OUTPUT: c_int = 1;
pub const CWEVENT_MASK: c_ulong = 0x0800;
pub const CWCOLORMAP: c_ulong = 0x2000;
pub const EXPOSURE_MASK: c_long = 0x0000_8000;
pub const ALLOC_NONE: c_int = 0;
pub const PROP_MODE_REPLACE: c_int = 0;
pub const KEY_RELEASE: c_int = 3;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(1);
}

global_asm! {
    ".global _start",
    "_start:",
    "mov rdi, rsp",
    "call main"
}

fn exit(status: i32) -> ! {
    unsafe {
        asm!(
        "syscall",
        in("rax") 60,
        in("rdi") status,
        options(noreturn)
        );
    }
}

unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    let r0;
    asm!(
    "syscall",
    inlateout("rax") 1isize => r0,
    in("rdi") fd,
    in("rsi") buf,
    in("rdx") count,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack, preserves_flags)
    );
    r0
}

// unsafe fn sleep(secs: u64) {
//     asm!(
//     "mov rsi, 0", // ; we wont use the second sleep arg, pass null to syscall
//     "sub rsp, 16", //
//     "mov QWORD PTR [rsp], rdi", // ; number of seconds the caller requested
//     "mov QWORD PTR [rsp + 8], rsi", // ; we won't use the nanoseconds
//     "mov rdi, rsp", // ; pass the struct that's on the stack to
//     "mov rax, 35", // ; sys_nanosleep
//     "syscall",
//     "add rsp, 16", // ; clean up stack
//     in("rdi") secs,
//     options(nostack, preserves_flags)
//     );
// }

macro_rules! intern_atom {
    ($display:ident, $atom_name:ident) => {{
        let atom_str = concat!(stringify!($atom_name), "\0");
        let atom = Xlib::XInternAtom(
            $display,
            atom_str.as_ptr() as *const _,
            FALSE,
        );
        atom
    }};
}

#[no_mangle]
unsafe fn main_minimal(_stack_top: *const u8) -> ! {
    let p: *const i8 = ptr::null();

    let display = Xlib::XOpenDisplay(p);

    if display.is_null() {
        panic!("XOpenDisplay failed");
    }

    // Shut down.
    Xlib::XCloseDisplay(display);

    exit(0);
}

#[no_mangle]
unsafe fn main(_stack_top: *const u8) -> ! {
    let p: *const i8 = ptr::null();

    let display = Xlib::XOpenDisplay(p);

    if display.is_null() {
        panic!("XOpenDisplay failed");
    }

    let glx_display = display as *mut glx::Display;
    // println!("GL Display: {:?}", glx_display);

    // Create window.
    let screen = Xlib::XDefaultScreen(display);
    // let root = Xlib::XRootWindow(display, screen);

    let mut glx_major = 0;
    let mut glx_minor = 0;
    let glx_result = glx::glXQueryVersion(glx_display, &mut glx_major, &mut glx_minor);
    // println!(
    //     "glX version: Major: {}, minor: {}, result: {}",
    //     glx_major, glx_minor, glx_result
    // );

    let attribute_list = &mut [
        glx::GLX_RGBA as c_int,
        glx::GLX_RED_SIZE as c_int,
        8,
        glx::GLX_GREEN_SIZE as c_int,
        8,
        glx::GLX_BLUE_SIZE as c_int,
        8,
        glx::None as c_int,
    ];

    let visual_info = glx::glXChooseVisual(glx_display, screen, attribute_list.as_mut_ptr());
    // println!("Visual info {:?}", visual_info);

    let root = Xlib::XRootWindow(display, (*visual_info).screen);
    // println!("Root window: {}", root);

    let color_map = Xlib::XCreateColormap(
        display,
        root,
        (*visual_info).visual as *mut Xlib::Visual,
        ALLOC_NONE,
    );
    // println!("Color map: {}", color_map);

    let _attributes = mem::MaybeUninit::<Xlib::XSetWindowAttributes>::uninit();
    let mut attributes: Xlib::XSetWindowAttributes = _attributes.assume_init();

    attributes.colormap = color_map;
    attributes.event_mask = EXPOSURE_MASK | KEY_PRESS_MASK;
    attributes.background_pixel = Xlib::XWhitePixel(display, (*visual_info).screen);
    let window_flags_enabled = CWCOLORMAP | CWEVENT_MASK | CWBACK_PIXEL;

    // attributes.background_pixel = Xlib::XWhitePixel(display, screen);

    // let display_width: u32 = Xlib::XDisplayWidth(display, screen).try_into().unwrap();
    // let display_height: u32 = Xlib::XDisplayHeight(display, screen).try_into().unwrap();

    /* make the new window occupy 1/9 of the screen's size. */
    // let width: u32 = display_width / 1;
    // let height: u32 = display_height / 1;
    // println!("window width - '{}'; height - '{}'\n", width, height);
    // println!("(*visual_info).depth: {:?}", (*visual_info).depth);

    let window = Xlib::XCreateWindow(
        display,
        root,
        0,
        0,
        1920,
        1080,
        0,
        (*visual_info).depth,
        INPUT_OUTPUT as c_uint,
        (*visual_info).visual as *mut Xlib::Visual,
        window_flags_enabled,
        &mut attributes,
        // 0,
        // 0,
        // INPUT_OUTPUT as u32,
        // ptr::null_mut(),
        // CWBACK_PIXEL,
        // &mut attributes,
    );

    Xlib::XMapWindow(display, window);

    let _net_wm_state_atom = intern_atom!(display, _NET_WM_STATE);
    let _net_wm_state_fullscreen_atom = intern_atom!(display, _NET_WM_STATE_FULLSCREEN);

    let _net_wm_allowed_atom = intern_atom!(display, _NET_WM_ALLOWED_ACTIONS);
    let _net_wm_action_fullscreen_atom = intern_atom!(display, _NET_WM_ACTION_FULLSCREEN);
    let wm_a_atom = intern_atom!(display, ATOM);

    let _net_wm_state_remove = 0; /* remove/unset property */
    let _net_wm_state_add = 1; /* add/set property */
    let _net_wm_state_toggle = 2; /* toggle property  */

    Xlib::XChangeProperty(
        display,
        window,
        _net_wm_allowed_atom,
        wm_a_atom,
        32,
        PROP_MODE_REPLACE as c_int,
        &_net_wm_action_fullscreen_atom as *const libc::c_ulong as *const libc::c_uchar,
        1,
    );
    Xlib::XChangeProperty(
        display,
        window,
        _net_wm_state_atom,
        wm_a_atom,
        32,
        PROP_MODE_REPLACE as c_int,
        &_net_wm_state_fullscreen_atom as *const libc::c_ulong as *const libc::c_uchar,
        1,
    );

    let gl_context = glx::glXCreateContext(
        glx_display,
        visual_info,
        ptr::null_mut(),
        gl::GL_TRUE as i32,
    );
    // println!("GL Context: {:?}", gl_context);
    let result = glx::glXMakeCurrent(glx_display, window, gl_context);
    // println!("glXMakeCurrent result: {:?}", result);

    gl::glEnable(gl::GL_DEPTH_TEST);
    gl_wrapper::load_extensions();

    // let gl_version = gl_wrapper::glGetString(gl::GL_VERSION);
    // println!("Version: {:?}", gl_version);

    // let gl_renderer = gl_wrapper::glGetString(gl::GL_RENDERER);
    // println!("Renderer: {:?}", gl_renderer);

    // let gl_vendor = gl_wrapper::glGetString(gl::GL_VENDOR);
    // println!("Vendor: {:?}", gl_vendor);

    // Set window title.
    // let title_str = CString::new("hello-world").unwrap();
    // Xlib::XStoreName(display, window, title_str.as_ptr() as *mut c_char);

    // Hook close requests.
    // let wm_protocols = intern_atom!(display, WM_PROTOCOLS);
    // let wm_protocols = Xlib::XInternAtom(display, "wm_delete_window".as_ptr() as *const _, FALSE);
    // let wm_delete_window: Xlib::Atom = intern_atom!(display, WM_DELETE_WINDOW);
    //
    // let mut protocols = [wm_delete_window];
    //
    // Xlib::XSetWMProtocols(
    //     display,
    //     window,
    //     protocols.as_mut_ptr(),
    //     protocols.len() as c_int,
    // );

    // Show window.
    // Xlib::XSelectInput(display, window, KEY_PRESS_MASK | KEY_RELEASE_MASK);

    // let gc = create_gc(display, window, false);
    // Xlib::XSync(display, FALSE);

    /* draw one pixel near each corner of the window */
    // draw_point(display, window, gc, 5, 5);
    // draw_point(display, window, gc, 5, (height - 5) as c_int);
    // draw_point(display, window, gc, (width - 5) as c_int, 5);
    // draw_point(display, window, gc, (width - 5) as c_int, (height - 5) as c_int);


    // Main loop.
    let _event = mem::MaybeUninit::<Xlib::XEvent>::uninit();
    let mut event: Xlib::XEvent = _event.assume_init();
    glx::glXSwapBuffers(display as *mut bindings::glx::_XDisplay, window);

    loop {
        let mut alt_pressed = false;
        Xlib::XNextEvent(display, &mut event);

        if event.type_.as_ref() == &KEY_PRESS {
            // 9 is esc
            if event.xkey.as_ref().keycode == 9 {
                write(1, b"esc\n".as_ptr(), 4);
                break;
            }
        }
    }

    // clean up
    // println!("cleanup ");
    glx::glXMakeCurrent(glx_display, glx::None.into(), ptr::null_mut());
    glx::glXDestroyContext(glx_display, gl_context);

    // Shut down.
    // println!("closing window");
    Xlib::XDestroyWindow(display, window);
    // println!("closing display");
    Xlib::XCloseDisplay(display);

    // println!("exit 0");
    exit(0);
}

// unsafe fn draw_point(display: *mut Xlib::Display, win: c_ulong, gc: Xlib::GC, x: c_int, y: c_int) {
//     Xlib::XDrawPoint(display, win, gc, x.try_into().unwrap(), y.try_into().unwrap());
// }
//
// unsafe fn create_gc(display: *mut Xlib::Display, win: Xlib::Window, reverse_video: bool) -> Xlib::GC  {
//     let valuemask: u64 = 0;
//     let mut values: Xlib::XGCValues = Xlib::XGCValues {
//         function: 0,
//         plane_mask: 0,
//         foreground: 0,
//         background: 0,
//         line_width: 0,
//         line_style: 0,
//         cap_style: 0,
//         join_style: 0,
//         fill_style: 0,
//         fill_rule: 0,
//         arc_mode: 0,
//         tile: 0,
//         stipple: 0,
//         ts_x_origin: 0,
//         ts_y_origin: 0,
//         font: 0,
//         subwindow_mode: 0,
//         graphics_exposures: 0,
//         clip_x_origin: 0,
//         clip_y_origin: 0,
//         clip_mask: 0,
//         dash_offset: 0,
//         dashes: 0,
//     };
//     let line_width: c_uint = 2;                   /* line width for the GC. */
//     let line_style = LINE_SOLID;             /* style for lines drawing and */
//     let cap_style = CAP_BUTT;                /* style of the line's edge and */
//     let join_style = JOIN_BEVEL;             /*  joined lines. */
//     let screen_num = Xlib::XDefaultScreen(display);
//
//     let gc = Xlib::XCreateGC(display, win, valuemask, &mut values);
//     if gc == ptr::null_mut() {
//         panic!("XCreateGC");
//     }
//
//     /* allocate foreground and background colors for this GC. */
//     if reverse_video {
//         Xlib::XSetForeground(display, gc, Xlib::XWhitePixel(display, screen_num));
//         Xlib::XSetBackground(display, gc, Xlib::XBlackPixel(display, screen_num));
//     }
//     else {
//         Xlib::XSetForeground(display, gc, Xlib::XBlackPixel(display, screen_num));
//         Xlib::XSetBackground(display, gc, Xlib::XWhitePixel(display, screen_num));
//     }
//
//     /* define the style of lines that will be drawn using this GC. */
//     Xlib::XSetLineAttributes(display, gc, line_width, line_style, cap_style, join_style);
//
//     /* define the fill style for the GC. to be 'solid filling'. */
//     Xlib::XSetFillStyle(display, gc, FILL_SOLID);
//
//     gc
// }
