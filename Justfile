release-all: (release "cash") (release "mtt-table") (release "mtt")

release dir: (build dir) (optimize dir)

test:
    cargo test

build dir:
    cargo build -r --target wasm32-unknown-unknown -p race-holdem-{{kebabcase(dir)}}

optimize dir:
    wasm-opt -Oz target/wasm32-unknown-unknown/release/race_holdem_{{snakecase(dir)}}.wasm -o target/race_holdem_{{snakecase(dir)}}.wasm
