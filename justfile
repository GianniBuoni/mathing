run bin="server" *args:
  cargo run -p "mathing-{{bin}}" -- {{args}}

[arg("package", short="p")]
@test package="":
  if [ "{{package}}" != "" ]; then \
    cargo test -p "mathing-{{package}}"; \
  else \
    cargo test; \
  fi

[arg("package", short="p")]
@lint package="":
  if [ "{{package}}" != "" ]; then \
    cargo fmt -p "mathing-{{package}}" --check; \
    cargo clippy \
    -p "mathing-{{package}}" \
    --all-targets \
    -- -D warnings; \
  else \
    cargo fmt --check; \
    cargo clippy --all-targets -- -D warnings; \
  fi

[default]
[arg("package", short="p")]
build package="": (test package)
  if [ "{{package}}" != "" ]; then \
    cargo build -p "mathing={{package}}"; \
  else \
    cargo build; \
  fi

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
    sqlx database create; \
    sqlx migrate run --source ./crates/mathing-server/migrations; \
  fi;

[working-directory: "crates/mathing-server"]
prepare:
  cargo sqlx prepare
