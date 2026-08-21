use std::fmt;
use error_stack::{Report, ResultExt};

// 1. Define an error type. It just needs Debug + Display + std::error::Error.
#[derive(Debug)]
struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "config error")
    }
}

impl std::error::Error for ConfigError {} // this replaces the old `Context` trait

// 2. A low-level function that fails and creates the initial Report.
fn read_file(path: &str) -> Result<String, Report<ConfigError>> {
    std::fs::read_to_string(path)
        .change_context(ConfigError)          // io::Error -> Report<ConfigError>
        .attach(format!("failed to read file `{path}`"))
}

// 3. A mid-level function that adds more context as it propagates.
fn load_config(path: &str) -> Result<String, Report<ConfigError>> {
    read_file(path)
        .attach("loading application config")
}

// 4. Top level: print the whole chain.
fn main() {
    if let Err(report) = load_config("config.toml") {
        eprintln!("{report:?}");
    }
}