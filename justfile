[default]
run bin="server":
  cargo run --bin "mathing"

test:
  cargo test

lint:
  cargo clippy -- -D warnings

build: test lint
  cargo test
