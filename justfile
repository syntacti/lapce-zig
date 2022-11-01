

alias b:= build
alias rb:= release-build

release-build:
  cargo build --target wasm32-wasi --release

build:
  cargo build --target wasm32-wasi

clean:
  cargo clean

fmt:
  cargo fmt

check:
  cargo check

clippy:
  cargo clippy