run bin="server" *args:
  cargo run -p "mathing-{{bin}}" -- {{args}}

test:
  cargo test

lint:
  cargo fmt --check
  cargo clippy --all-targets -- -D warnings

[default]
build: test
  cargo build

@start_db:
  if [ "$(pg_ctl status | grep "is running")" ]; then \
    echo "PG server already running."; \
  else \
    pg_ctl start -l $PGDATA/logfile -o --unix_socket_directories=$PWD/$PGDATA; \
  fi;

@init_db:
  if [ "$(pg_ctl status | grep "is running")" ]; then \
    echo "PG server already initialized."; \
  else \
    pg_ctl init; \
    just start_db; \
  fi;
