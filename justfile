[default]
run bin="server":
  cargo run -p "mathing-{{bin}}"

test:
  cargo test

lint:
  cargo clippy -- -D warnings

build: test lint
  cargo test
