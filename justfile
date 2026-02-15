run bin="server" *args:
  cargo run -p "mathing-{{bin}}" -- {{args}}

test:
  cargo test

lint:
  cargo fmt --check
  cargo clippy -- -D warnings

[default]
build: test
  cargo build

@start_db:
  pg_check=$(pg_ctl status | grep "no server running"); \
  if [ "$pg_check" == "" ]; then \
    echo "PG server running!"; \
  else \
    pg_ctl start -l $PGDATA/logfile -o --unix_socket_directories=$PWD/$PGDATA; \
  fi
