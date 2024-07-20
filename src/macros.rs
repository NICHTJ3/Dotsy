/// macro to raise the error specified
#[macro_export]
macro_rules! dotsy_err {
    ($type:expr) => {
        Err($type)?
    };
}

/// macro to log a warning to the user
#[macro_export]
macro_rules! dotsy_log_warning {
    ($($arg:tt)*) => {{
        print!("{}: ",ansi_term::Color::Yellow.paint("WARN"));
        eprintln!($($arg)+)
    }};
}

/// macro to log an error to the user
#[macro_export]
macro_rules! dotsy_log_error {
    ($($arg:tt)*) => {{
        print!("{}: ",ansi_term::Color::Red.paint("ERROR"));
        println!($($arg)+)
    }};
}

/// macro to return an absolute path base on the relative_path from $XDG_CONFIG_HOME
#[macro_export]
macro_rules! xdg_config_home {
    ($relative_path:expr) => {
        match dirs::config_dir() {
            Some(base) => Some(base.join($relative_path)),
            None => None,
        }
    };
}

/// macro to return an absolute path base on the relative_path from $HOME
#[macro_export]
macro_rules! home {
    ($relative_path:expr) => {
        match dirs::home_dir() {
            Some(base) => Some(base.join($relative_path)),
            None => None,
        }
    };
}
