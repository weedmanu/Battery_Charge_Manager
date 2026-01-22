//! Debug and logging module for Battery Manager
//!
//! Provides conditional debug logging when --debug flag is enabled.
//! Traces UI events and core operations.

use std::sync::atomic::{AtomicBool, Ordering};

/// Global debug flag
static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable debug mode
pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::Relaxed);
}

/// Check if debug mode is enabled
pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

/// Debug macro - only prints when debug is enabled
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::core::debug::is_debug_enabled() {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

/// Trace a UI event
#[macro_export]
macro_rules! debug_ui {
    ($($arg:tt)*) => {
        if $crate::core::debug::is_debug_enabled() {
            eprintln!("[DEBUG UI] {}", format!($($arg)*));
        }
    };
}

/// Trace a core operation
#[macro_export]
macro_rules! debug_core {
    ($($arg:tt)*) => {
        if $crate::core::debug::is_debug_enabled() {
            eprintln!("[DEBUG CORE] {}", format!($($arg)*));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_disabled_by_default() {
        assert!(!is_debug_enabled());
    }

    #[test]
    fn test_enable_debug() {
        enable_debug();
        assert!(is_debug_enabled());
    }
}
