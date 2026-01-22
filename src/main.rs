//! Battery Manager - GTK4 Battery Management Application
//!
//! SOLID Architecture:
//! - core/ : Business logic (`BatteryInfo`, `PowerSupplyInfo`, vendor detection, i18n, debug)
//! - ui/ : User interface (GTK4, theme management)
//!
//! # Command-line arguments
//! - `--debug` : Enable debug mode with exhaustive tracing
//! - `--lang=en` : Set language to English (default: fr)
//! - `--lang=fr` : Set language to French

mod core;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;
use std::env;

const APP_ID: &str = "com.battery.manager";

fn main() {
    // Load or detect language preference
    let config_file = dirs::config_dir().map(|d| d.join("battery-manager").join("language.conf"));

    let mut lang_loaded = false;
    if let Some(ref config_path) = config_file {
        if let Ok(saved_lang) = std::fs::read_to_string(config_path) {
            let lang = saved_lang.trim();
            if lang == "en" || lang == "fr" {
                core::i18n::set_language(lang);
                lang_loaded = true;
            }
        }
    }

    // If no saved preference, detect system language
    if !lang_loaded {
        if let Ok(sys_lang) = env::var("LANG").or_else(|_| env::var("LC_ALL")) {
            let lang = if sys_lang.starts_with("en") {
                "en"
            } else {
                "fr" // Default to French
            };
            core::i18n::set_language(lang);
        }
    }

    // Load saved theme preference
    let theme_file = dirs::config_dir().map(|d| d.join("battery-manager").join("theme.conf"));

    if let Some(ref theme_path) = theme_file {
        if let Ok(saved_theme) = std::fs::read_to_string(theme_path) {
            let theme = saved_theme.trim();
            if theme == "dark" || theme == "light" {
                ui::theme::set_theme(theme);
            }
        }
    }

    // Parse command-line arguments and filter GTK arguments
    let args: Vec<String> = env::args().collect();
    let mut gtk_args = vec![args[0].clone()];

    for arg in &args[1..] {
        match arg.as_str() {
            "--debug" => {
                core::debug::enable_debug();
                crate::core::debug::debug_log("🚀 [MAIN] Debug mode enabled");
            }
            arg if arg.starts_with("--lang=") => {
                let lang = &arg[7..];
                core::i18n::set_language(lang);
                if core::debug::is_debug_enabled() {
                    crate::core::debug::debug_log_args(std::format_args!(
                        "🌐 [MAIN] Language set to: {lang}"
                    ));
                }
            }
            "--help" | "-h" => {
                println!("Battery Manager v{}", env!("CARGO_PKG_VERSION"));
                println!("\nUsage: battery-manager [OPTIONS]");
                println!("\nOptions:");
                println!("  --debug        Enable debug mode with exhaustive tracing");
                println!("  --lang=en      Set language to English");
                println!("  --lang=fr      Set language to French (default)");
                println!("  --help, -h     Show this help message");
                std::process::exit(0);
            }
            _ => {
                // Pass unrecognized args to GTK (like --display, etc)
                gtk_args.push(arg.clone());
            }
        }
    }

    crate::core::debug::debug_log("🚀 [MAIN] Starting Battery Manager application");
    crate::core::debug::debug_log_args(std::format_args!(
        "🌐 [MAIN] Current language: {}",
        core::i18n::get_language()
    ));

    // The application starts without root privileges
    // pkexec will be requested only when clicking "Apply settings"
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(ui::build_ui);

    crate::core::debug::debug_log("🖥️ [MAIN] Running GTK4 application");
    app.run_with_args(&gtk_args);
}
