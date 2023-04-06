# 4k intro in rust thingy

4k rust revision entry tryout, heavily inspired by @kpcyrd's blog post "Writing a Linux executable from scratch with x86_64-unknown-none and Rust" and based on  aleksanb's fourkay.

## requirements

`apt install llvm-dev libclang-dev clang libgl-dev nasm`

## build

* `$ git clone https://gitlab.com/PoroCYon/vondehi.git`
* `$ rustup target add x86_64-unknown-linux-gnu --toolchain nightly-x86_64-unknown-linux-gnu`
* `$ rustup component add rust-src --toolchain nightly`

