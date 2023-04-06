#!/bin/bash

#STRIP_SECTIONS="--strip-all -R '.note*' -R .comment"
STRIP_SECTIONS="--strip-all -R '.note*' -R .comment -R .eh_frame -R .eh_frame_hdr -R .fini -R .fini_array -R .init_array -R .got -R .data"

rm -rf build && mkdir build

cargo +nightly build --no-default-features -Z minimal-versions -Z build-std-features=panic_immediate_abort -Z build-std=panic_abort --target x86_64-unknown-linux-gnu --release

cp target/x86_64-unknown-linux-gnu/release/revision2023-4k-intro-rust build/revision2023-4k-intro-rust-pre-strip
wc --bytes build/revision2023-4k-intro-rust-pre-strip
strip $STRIP_SECTIONS build/revision2023-4k-intro-rust-pre-strip -o build/revision2023-4k-intro-rust-stripped
wc --bytes build/revision2023-4k-intro-rust-stripped

nasm -fbin -obuild/vondehi ../vondehi/vondehi.asm -DNO_CHEATING
lzma --best -c build/revision2023-4k-intro-rust-stripped > build/revision2023-4k-intro-rust-lzma
wc --bytes build/vondehi
wc --bytes build/revision2023-4k-intro-rust-lzma
cat build/vondehi build/revision2023-4k-intro-rust-lzma > build/revision2023-4k-intro-rust

chmod +x build/revision2023-4k-intro-rust
wc --bytes build/revision2023-4k-intro-rust
