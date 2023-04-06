extern crate bindgen;

// use std::collections::HashSet;
use std::env;

use std::path::PathBuf;

// #[derive(Debug)]
// struct IgnoreMacros(HashSet<String>);

// impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
//     fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
//         if self.0.contains(name) {
//             bindgen::callbacks::MacroParsingBehavior::Ignore
//         } else {
//             bindgen::callbacks::MacroParsingBehavior::Default
//         }
//     }
// }

struct Lib {
    pub link_name: String,
    pub header_path: String,
    pub out_name: String,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu"); // needed for -lX11 etc.
    // println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-arg=-nostdlib"); // does nothing
    println!("cargo:rustc-link-arg=-lX11");
    println!("cargo:rustc-link-arg=-lxcb");
    println!("cargo:rustc-link-arg=-lc");
    // println!("cargo:rustc-link-arg=-nostartfiles");
    // println!("cargo:rustc-target-feature=+crt-static");

    let libraries = [
        Lib {
            link_name: "X11".to_string(),
            header_path: "/usr/include/X11/Xlib.h".to_string(),
            out_name: "Xlib.rs".to_string(),
        },
        Lib {
            link_name: "GL".to_string(),
            header_path: "/usr/include/GL/gl.h".to_string(),
            out_name: "gl.rs".to_string(),
        },
        Lib {
            link_name: "GLX".to_string(),
            header_path: "/usr/include/GL/glx.h".to_string(),
            out_name: "glx.rs".to_string(),
        },
        // Lib {
        //     link_name: "m".to_string(),
        //     header_path: "/usr/include/math.h".to_string(),
        //     out_name: "math.rs".to_string(),
        // },
    ];

    for lib in &libraries {
        let mut bindings = bindgen::Builder::default()
            .rust_target(bindgen::RustTarget::Nightly)
            // .trust_clang_mangling(false)
            .derive_copy(false)
            .derive_debug(false)
            .use_core()
            .raw_line(format!(r#"#[link(name="{}")] extern{{}}"#, lib.link_name))
            // .ctypes_prefix("libc")
            .ctypes_prefix("::core::ffi")
            .header(lib.header_path.clone());

        // if lib.out_name == "math.rs" {
        //     let ignored_macros = IgnoreMacros(
        //         vec![
        //             "FP_INFINITE".into(),
        //             "FP_NAN".into(),
        //             "FP_NORMAL".into(),
        //             "FP_SUBNORMAL".into(),
        //             "FP_ZERO".into(),
        //             "IPPORT_RESERVED".into(),
        //         ]
        //         .into_iter()
        //         .collect(),
        //     );
        //
        //     bindings = bindings
        //         .parse_callbacks(Box::new(ignored_macros))
        //         .rustfmt_bindings(true);
        // }

        if lib.out_name == "Xlib.rs" {
            bindings = bindings
                .allowlist_function("XOpenDisplay")
                .allowlist_function("XDefaultScreen")
                .allowlist_function("XRootWindow")
                .allowlist_function("XChangeProperty")
                // .allowlist_function("XSetBackground")
                .allowlist_function("XWhitePixel")
                .allowlist_function("XCreateWindow")
                // .allowlist_function("XSelectInput")
                .allowlist_function("XInternAtom")
                .allowlist_function("XMapWindow")
                .allowlist_function("XCreateColormap")
                // .allowlist_function("XSetFillStyle")
                .allowlist_function("XNextEvent")
                // .allowlist_function("XCreateGC")
                // .allowlist_function("XBlackPixel")
                // .allowlist_function("XSetWMProtocols")
                // .allowlist_function("XClientMessageEvent")
                .allowlist_function("XDestroyWindow")
                // .allowlist_function("XDrawPoint")
                // .allowlist_function("XDisplayWidth")
                // .allowlist_function("XDisplayHeight")
                // .allowlist_function("XSync")
                // .allowlist_function("XSync")
                // .allowlist_function("XSync")
                // .allowlist_function("XSync")

                .allowlist_function("XCloseDisplay")
            ;
        }

        let bindings = bindings.generate().expect(&format!(
            "Couldn't generate bindings for {}",
            lib.header_path
        ));

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join(&lib.out_name))
            .expect("Couldn't write bindings!");
    }

    // generate_bindings();
}

// MIT License: https://github.com/diwic/alsa-sys/blob/master/build.rs

// fn generate_bindings() {
//     let mut codegen_config = bindgen::CodegenConfig::empty();
//     codegen_config.insert(bindgen::CodegenConfig::FUNCTIONS);
//     codegen_config.insert(bindgen::CodegenConfig::TYPES);
//
//     let builder = bindgen::Builder::default()
//         .rust_target(bindgen::RustTarget::Nightly)
//         .derive_copy(false)
//         .derive_debug(false)
//         .use_core()
//         .raw_line(r#"#[link(name="asound")] extern {}"#)
//         .header("/usr/include/alsa/asoundlib.h")
//         .size_t_is_usize(true)
//         .prepend_enum_name(false)
//         .layout_tests(false)
//         .ctypes_prefix("::core::ffi")
//         //.whitelist_function("snd_.*")
//         //.whitelist_type("_?snd_.*")
//         //.whitelist_type(".*va_list.*")
//         ;
//     let bindings = builder.generate().expect("Unable to generate bindings");
//
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//
//     bindings
//         .write_to_file(out_path.join("alsa.rs"))
//         .expect("Couldn't write bindings");
// }
