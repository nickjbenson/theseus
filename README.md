# Theseus #

Currently individual testing projects within while the project gets started.

## Compile cross-platform when using gfx-rs/rendy/amethyst using a nightly Cargo feature ##

When https://github.com/rust-lang/cargo/issues/7914 stabilizes, we should just be able to `cargo build` or `cargo run` with the following Cargo.toml and not have to add `--features metal` or `--features vulkan` all the time when building for MacOS vs Linux & Windows.

Cargo.toml:
```toml
[target.'cfg(not(target_os = "macos"))'.dependencies.amethyst]
version = "0.15"
features = ["vulkan"]
[target.'cfg(target_os = "macos")'.dependencies.amethyst]
version = "0.15"
features = ["metal"]
```

This allows the same cargo build/run commands because Metal needs to be used as the graphics API for MacOS builds.

run:
```sh
cargo +nightly run -Z features=itarget
```

build:
```sh
cargo +nightly build -Z features=itarget
```
