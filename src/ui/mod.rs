//! User interface module for GTK4 application
//!
//! Contains main window, information tab, settings tab, UI preferences tab, theme management, and reusable components.

pub mod app;
pub mod components;
pub mod info_tab;
pub mod settings_tab;
pub mod theme;
pub mod ui_tab;

pub use app::build_ui;
