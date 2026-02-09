run bin="server" *args:
  cargo run -p "mathing-{{bin}}" -- {{args}}

test:
  cargo test

lint:
  cargo clippy -- -D warnings

[default]
build: test lint
  cargo build

start_db:
  pg_ctl start -l .postgres/logfile -o --unix_socket_directories=$PWD/.postgres
