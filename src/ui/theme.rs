//! Theme management for dark/light mode
//!
//! Applies CSS styling for dark theme while preserving default light theme.

use std::sync::RwLock;

/// Current theme setting
static CURRENT_THEME: RwLock<String> = RwLock::new(String::new());

/// Set the current theme
pub fn set_theme(theme: &str) {
    *CURRENT_THEME.write().unwrap() = theme.to_string();
}

/// Get the current theme
pub fn get_theme() -> String {
    CURRENT_THEME.read().unwrap().clone()
}

/// Apply dark theme CSS to the application
pub fn apply_dark_theme() {
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        "
        window {
            background-color: #252525;
            color: #d5d5d5;
        }
        
        * {
            border-color: #3a3a3a;
        }
        
        notebook {
            background-color: #2d2d2d;
            border: none;
        }
        
        notebook > header {
            background-color: #282828;
            border: none;
        }
        
        notebook > header > tabs > tab {
            background-color: #242424;
            color: #b0b0b0;
            border: 1px solid #333333;
        }
        
        notebook > header > tabs > tab:checked {
            background-color: #2d2d2d;
            color: #e8e8e8;
            border-bottom: none;
        }
        
        notebook > stack {
            background-color: #2d2d2d;
            border: none;
        }
        
        frame {
            background-color: #323232;
            border: 1px solid #3a3a3a;
        }
        
        frame > box {
            background-color: #323232;
        }
        
        label {
            color: #d5d5d5;
        }
        
        box {
            background-color: #252525;
        }
        
        scrolledwindow {
            background-color: #2d2d2d;
        }
        
        scrolledwindow > *  {
            background-color: #2d2d2d;
        }
        
        separator {
            background-color: #383838;
        }
        
        spinbutton {
            background-color: #323232;
            color: #d5d5d5;
            border: 1px solid #3a3a3a;
        }
        
        spinbutton entry {
            background-color: #323232;
            color: #d5d5d5;
        }
        
        button {
            background-color: #383838;
            color: #d5d5d5;
            border: 1px solid #454545;
        }
        
        button:hover {
            background-color: #424242;
        }
        
        switch {
            background-color: #383838;
        }
        
        switch:checked {
            background-color: #0d7377;
        }
        
        switch slider {
            background-color: #656565;
        }
        ",
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to display"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Apply light theme CSS (reproduces default GTK4 Adwaita theme)
pub fn apply_light_theme() {
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        "
        window {
            background-color: #f2f2f2;
            color: #2e3436;
        }
        
        * {
            border-color: #d4d4d4;
        }
        
        notebook {
            background-color: #f8f8f8;
            border: none;
        }
        
        notebook > header {
            background-color: #ececec;
            border: none;
        }
        
        notebook > header > tabs > tab {
            background-color: #e6e6e6;
            color: #5e5c64;
            border: 1px solid #d0d0d0;
        }
        
        notebook > header > tabs > tab:checked {
            background-color: #f8f8f8;
            color: #2e3436;
        }
        
        notebook > stack {
            background-color: #f8f8f8;
            border: none;
        }
        
        frame {
            background-color: #fcfcfc;
            border: 1px solid #d8d8d8;
        }
        
        frame > box {
            background-color: #fcfcfc;
        }
        
        label {
            color: #2e3436;
        }
        
        box {
            background-color: #f2f2f2;
        }
        
        scrolledwindow {
            background-color: #f8f8f8;
        }
        
        separator {
            background-color: #d4d4d4;
        }
        
        spinbutton {
            background-color: #fcfcfc;
            color: #2e3436;
            border: 1px solid #d4d4d4;
        }
        
        spinbutton entry {
            background-color: #fcfcfc;
            color: #2e3436;
        }
        
        button {
            background-color: #ececec;
            color: #2e3436;
            border: 1px solid #d0d0d0;
        }
        
        button:hover {
            background-color: #e4e4e4;
        }
        
        switch {
            background-color: #d4d4d4;
        }
        
        switch:checked {
            background-color: #3584e4;
        }
        
        switch slider {
            background-color: #f5f5f5;
        }
        ",
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to display"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Apply theme based on current setting
pub fn apply_current_theme() {
    let theme = get_theme();
    if theme == "dark" {
        apply_dark_theme();
    } else {
        apply_light_theme();
    }
}
