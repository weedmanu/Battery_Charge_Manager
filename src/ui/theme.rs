//! Theme management for dark/light mode
//!
//! Applies CSS styling for dark theme while preserving default light theme.

use std::sync::RwLock;

static CURRENT_THEME: RwLock<String> = RwLock::new(String::new());

pub fn set_theme(theme: &str) {
    *CURRENT_THEME.write().expect("Theme RwLock poisoned") = theme.to_string();
}

pub fn get_theme() -> String {
    CURRENT_THEME.read().expect("Theme RwLock poisoned").clone()
}

/// Applies CSS theme with given colors
fn apply_theme_css(is_dark: bool) {
    let (bg, fg, frame_bg, border, note_bg, note_border, note_text) = if is_dark {
        (
            "#252525", "#d5d5d5", "#323232", "#3a3a3a", "#1e3a52", "#2d5373", "#a8c8e8",
        )
    } else {
        (
            "#f6f5f4", "#2e3436", "#ffffff", "#d0d0d0", "#e3f2fd", "#90caf9", "#1976d2",
        )
    };

    let (primary, success, warning, danger) = if is_dark {
        ("#5dade2", "#6ec56e", "#ffb84d", "#ff6b6b")
    } else {
        ("#2196f3", "#4caf50", "#ff9800", "#f44336")
    };

    let css = format!("
        window {{ background-color: {bg}; color: {fg}; }}
        * {{ border-color: {border}; }}
        .color-primary {{ color: {primary}; }}
        .color-success {{ color: {success}; }}
        .color-warning {{ color: {warning}; }}
        .color-danger {{ color: {danger}; }}
        .info-note {{ background-color: {note_bg}; border: 1px solid {note_border}; border-radius: 6px; }}
        .info-note box {{ background-color: {note_bg}; }}
        .info-note label {{ color: {note_text}; background-color: transparent; }}
        notebook, notebook > stack {{ background-color: {bg}; border: none; }}
        notebook > header {{ background-color: {bg}; border: none; }}
        notebook > header > tabs > tab {{ background-color: {frame_bg}; color: {fg}; border: 1px solid {border}; }}
        notebook > header > tabs > tab:checked {{ background-color: {bg}; border-bottom: none; }}
        frame {{ background-color: {frame_bg}; border: 1px solid {border}; }}
        frame box {{ background-color: {frame_bg}; }}
        frame label {{ background-color: transparent; color: {fg}; }}
        label {{ color: {fg}; background-color: transparent; }}
        box {{ background-color: {bg}; }}
        scrolledwindow {{ background-color: {bg}; }}
        scrolledwindow box {{ background-color: {bg}; }}
        separator {{ background-color: {border}; }}
        spinbutton, spinbutton entry {{ background-color: {frame_bg}; color: {fg}; border: 1px solid {border}; }}
        button {{ background-color: {frame_bg}; color: {fg}; border: 1px solid {border}; }}
        button:hover {{ background-color: {border}; }}
        switch {{ background-color: {frame_bg}; }}
        switch:checked {{ background-color: {success}; }}
    ");

    let provider = gtk4::CssProvider::new();
    provider.load_from_data(&css);
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Display required"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn apply_dark_theme() {
    apply_theme_css(true);
    crate::core::debug::debug_log("🌙 [THEME] Dark theme applied");
}

pub fn apply_light_theme() {
    apply_theme_css(false);
    crate::core::debug::debug_log("☀️ [THEME] Light theme applied");
}

pub fn apply_current_theme() {
    let theme = get_theme();
    if theme == "dark" {
        apply_dark_theme();
    } else {
        apply_light_theme();
    }
}
