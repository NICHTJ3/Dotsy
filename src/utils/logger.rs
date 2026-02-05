/// Centralized logging utilities for Dotsy
use ansi_term::Color;

/// Log level for messages
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Success,
}

/// Logger struct for consistent logging across the application
pub struct Logger;

impl Logger {
    /// Log an error message
    pub fn error(message: &str) {
        eprintln!("{}: {}", Color::Red.paint("ERROR"), message);
    }

    /// Log a warning message
    pub fn warning(message: &str) {
        eprintln!("{}: {}", Color::Yellow.paint("WARN"), message);
    }

    /// Log an info message
    pub fn info(message: &str) {
        println!("{}: {}", Color::Blue.paint("INFO"), message);
    }

    /// Log a success message
    pub fn success(message: &str) {
        println!("{}: {}", Color::Green.paint("SUCCESS"), message);
    }

    /// Log a formatted message with a custom level
    pub fn log(level: LogLevel, message: &str) {
        match level {
            LogLevel::Error => Self::error(message),
            LogLevel::Warning => Self::warning(message),
            LogLevel::Info => Self::info(message),
            LogLevel::Success => Self::success(message),
        }
    }

    /// Log an action being attempted
    pub fn attempting(action: &str, target: &str) {
        println!(
            "{} {}: {}",
            Color::Purple.paint("Attempting"),
            action,
            target
        );
    }
}
