# learn rust
## init project
```
cargo new hello && cd hello && cargo run
```
1. Cargo.toml: 包含项目的原信息和依赖。
2. Cargo.lock: CARGO生成的文件
3. src: 源文件文件夹
4. target: 目标文件夹

## Build with rustc
可以直接使用编译器编译：
```
rustc ./src/main.rs && ./mian
```
## Running test
```
cargo test
```
## Extea tools
Two useful utilities are the rustfmt tool (for automatically formatting your code) and clippy (for getting code advice). Note that clippy is still a work in progress, and sometimes gives false positives. To get them set up, run:
```
$ rustup component add clippy-preview rustfmt-preview
```
And then you can run them with:
```
$ cargo fmt
$ cargo clippy
```
