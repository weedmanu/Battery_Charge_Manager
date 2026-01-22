//! Debug and logging module for Battery Manager
//!
//! Provides conditional debug logging when --debug flag is enabled.
//! Traces UI events and core operations.

use std::fmt;
use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};

/// Global debug flag
static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable debug mode
pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::Relaxed);
}

/// Disable debug mode (tests only)
#[cfg(test)]
pub fn disable_debug() {
    DEBUG_ENABLED.store(false, Ordering::Relaxed);
}

/// Check if debug mode is enabled
pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LogColor {
    None,
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

fn color_mode() -> ColorMode {
    if std::env::var_os("NO_COLOR").is_some() {
        return ColorMode::Never;
    }

    // Common convention: force color when set.
    if std::env::var_os("CLICOLOR_FORCE").is_some() {
        return ColorMode::Always;
    }

    match std::env::var("BATTERY_MANAGER_COLOR") {
        Ok(v) => match v.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "always" => ColorMode::Always,
            "0" | "false" | "no" | "never" => ColorMode::Never,
            _ => ColorMode::Auto,
        },
        Err(_) => ColorMode::Auto,
    }
}

fn should_colorize_stderr() -> bool {
    match color_mode() {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => std::io::stderr().is_terminal(),
    }
}

fn detect_color_from_text(text: &str) -> LogColor {
    let lower = text.to_ascii_lowercase();

    if text.contains("[ERROR]")
        || text.contains('❌')
        || lower.contains(" error")
        || lower.contains("error:")
        || lower.contains("failed")
    {
        return LogColor::Error;
    }

    if text.contains("[WARN]")
        || text.contains("[WARNING]")
        || text.contains("WARNING")
        || text.contains('⚠')
    {
        return LogColor::Warning;
    }

    LogColor::None
}

fn ensure_marker(line: String, color: LogColor) -> String {
    match color {
        LogColor::Error => {
            if !line.contains('❌') {
                if let Some(rest) = line.strip_prefix("[DEBUG] ") {
                    return format!("[DEBUG] ❌ {rest}");
                }
                if let Some(rest) = line.strip_prefix("[ERROR] ") {
                    return format!("[ERROR] ❌ {rest}");
                }
            }
        }
        LogColor::Warning => {
            if !line.contains('⚠') {
                if let Some(rest) = line.strip_prefix("[DEBUG] ") {
                    return format!("[DEBUG] ⚠️ {rest}");
                }
                if let Some(rest) = line.strip_prefix("[WARN] ") {
                    return format!("[WARN] ⚠️ {rest}");
                }
            }
        }
        LogColor::None => {}
    }

    line
}

fn colorize_line(color: LogColor, line: &str) -> String {
    match color {
        LogColor::None => line.to_string(),
        LogColor::Error => format!("\u{001b}[31m{line}\u{001b}[0m"),
        // "Orange" terminal-friendly: usually renders as yellow; avoids relying on 256-color.
        LogColor::Warning => format!("\u{001b}[33m{line}\u{001b}[0m"),
    }
}

/// Log a debug message (function version for easier use)
pub fn debug_log(message: &str) {
    if is_debug_enabled() {
        let line = format!("[DEBUG] {message}");
        let color = detect_color_from_text(&line);
        let line = ensure_marker(line, color);

        if should_colorize_stderr() {
            eprintln!("{}", colorize_line(color, &line));
            return;
        }

        eprintln!("{line}");
    }
}

/// Log a debug message without allocating a temporary `String`.
pub fn debug_log_args(args: fmt::Arguments<'_>) {
    if is_debug_enabled() {
        let line = format!("[DEBUG] {args}");
        let color = detect_color_from_text(&line);
        let line = ensure_marker(line, color);

        if should_colorize_stderr() {
            eprintln!("{}", colorize_line(color, &line));
            return;
        }

        eprintln!("{line}");
    }
}

/// Log an error to stderr (always) without forcing a temporary `String` at callsite.
pub fn terminal_error_args(args: fmt::Arguments<'_>) {
    let line = format!("[ERROR] {args}");
    let line = ensure_marker(line, LogColor::Error);

    if should_colorize_stderr() {
        eprintln!("{}", colorize_line(LogColor::Error, &line));
        return;
    }

    eprintln!("{line}");
}

/// Debug macro - only prints when debug is enabled
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::core::debug::debug_log_args(std::format_args!(
            "🔧 [LOG] {}",
            std::format_args!($($arg)*)
        ));
    };
}

/// Trace a UI event
#[macro_export]
macro_rules! debug_ui {
    ($($arg:tt)*) => {
        $crate::core::debug::debug_log_args(std::format_args!(
            "🧭 [UI] {}",
            std::format_args!($($arg)*)
        ));
    };
}

/// Trace a core operation
#[macro_export]
macro_rules! debug_core {
    ($($arg:tt)*) => {
        $crate::core::debug::debug_log_args(std::format_args!(
            "🧠 [CORE] {}",
            std::format_args!($($arg)*)
        ));
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_debug_disabled_by_default() {
        disable_debug();
        assert!(!is_debug_enabled());
    }

    #[test]
    fn test_enable_debug() {
        disable_debug();
        enable_debug();
        assert!(is_debug_enabled());
    }

    #[test]
    fn test_source_debug_log_literals_are_tagged() {
        fn visit_rs_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
            let Ok(entries) = fs::read_dir(dir) else {
                return;
            };

            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_rs_files(&path, files);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        visit_rs_files(&root, &mut files);

        for file in files {
            let Ok(text) = fs::read_to_string(&file) else {
                continue;
            };

            for (idx, line) in text.lines().enumerate() {
                let Some(pos) = line.find("debug_log(\"") else {
                    continue;
                };

                let rest = &line[pos + "debug_log(\"".len()..];
                let Some(end) = rest.find("\"") else {
                    continue;
                };

                let message = &rest[..end];
                assert!(
                    message.contains('[') && message.contains(']'),
                    "{}:{} debug_log literal missing [TAG]: {message}",
                    file.display(),
                    idx + 1
                );
            }
        }
    }
}
