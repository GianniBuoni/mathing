use env_logger::{Builder, DEFAULT_WRITE_STYLE_ENV, Env};

pub fn logger_init() {
    let env = Env::default()
        .filter("LOG_LEVEL")
        .write_style(DEFAULT_WRITE_STYLE_ENV);

    Builder::from_env(env).init()
}
