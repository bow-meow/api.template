# you need just, nushell and rust installed to use this
# use the three following commands in order:
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# cargo install nu
# cargo install just
set shell := ["nu", "-c"]

rbuildr: rbuild rrun
rbuild:
    cargo build --release
build:
    cargo build
rrun:
    cargo run --release
run:
    cargo run
buildr: build run
clean:
    cargo clean
fetch:
    cargo fetch
lint:
    cargo clippy
lintf:
    cargo clippy --fix
clear-cache:
    cargo cache -r all
