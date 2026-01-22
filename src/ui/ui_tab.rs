//! UI preferences tab for language and theme settings
//!
//! Allows users to switch between languages and themes with live preview.

use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation, ScrolledWindow, Switch};

use crate::core::i18n::t;
use crate::ui::components::InfoCard;

/// Builds the UI preferences tab content
///
/// # Returns
///
/// `ScrolledWindow` containing language and theme controls
#[allow(clippy::too_many_lines)]
pub fn build_ui_tab() -> ScrolledWindow {
    crate::core::debug::debug_log("🎛️ [UI_TAB] Building UI preferences tab");
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let content_box = Box::new(Orientation::Vertical, 12);
    content_box.set_margin_top(15);
    content_box.set_margin_bottom(15);
    content_box.set_margin_start(15);
    content_box.set_margin_end(15);

    // === Card Langue ===
    let (lang_frame, lang_box) = InfoCard::create(&format!("🌐 {}", t("language_setting")));
    lang_box.set_spacing(10);

    let lang_row = Box::new(Orientation::Horizontal, 10);
    lang_row.set_halign(gtk4::Align::Center);

    let lang_fr_label = Label::new(Some(&t("language_fr")));
    lang_fr_label.set_markup(&format!("<span size='large'>{}</span>", t("language_fr")));

    let lang_switch = Switch::new();
    lang_switch.set_active(crate::core::i18n::get_language() == "en");
    lang_switch.set_valign(gtk4::Align::Center);
    lang_switch.set_margin_start(15);
    lang_switch.set_margin_end(15);

    let lang_en_label = Label::new(Some(&t("language_en")));
    lang_en_label.set_markup(&format!("<span size='large'>{}</span>", t("language_en")));

    lang_row.append(&lang_fr_label);
    lang_row.append(&lang_switch);
    lang_row.append(&lang_en_label);
    lang_box.append(&lang_row);

    let lang_status = Label::new(None);
    lang_status.set_halign(gtk4::Align::Center);
    lang_status.set_margin_top(10);
    lang_box.append(&lang_status);

    lang_switch.connect_state_set(glib::clone!(
        #[weak]
        lang_status,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_switch, state| {
            let new_lang = if state { "en" } else { "fr" };

            crate::core::debug::debug_log_args(std::format_args!(
                "🌐 [UI_TAB] Language switch toggled -> {new_lang}"
            ));
            crate::core::i18n::set_language(new_lang);

            // Save to config file
            if let Some(config_dir) = dirs::config_dir() {
                let app_config_dir = config_dir.join("battery-manager");
                let _ = std::fs::create_dir_all(&app_config_dir);
                let config_file = app_config_dir.join("language.conf");
                let _ = std::fs::write(config_file, new_lang);
                crate::core::debug::debug_log_args(std::format_args!(
                    "💾 [UI_TAB] Saved language.conf -> {new_lang}"
                ));
            }

            lang_status.set_markup(&format!(
                "<span size='small'>{}</span>",
                t("restart_required")
            ));
            lang_status.add_css_class("color-warning");

            glib::Propagation::Proceed
        }
    ));

    content_box.append(&lang_frame);

    // === Card Thème ===
    let (theme_frame, theme_box) = InfoCard::create(&format!("🎨 {}", t("theme_setting")));
    theme_box.set_spacing(10);

    let theme_row = Box::new(Orientation::Horizontal, 10);
    theme_row.set_halign(gtk4::Align::Center);

    let theme_light_label = Label::new(Some(&t("theme_light")));
    theme_light_label.set_markup(&format!(
        "<span size='large'>☀️ {}</span>",
        t("theme_light")
    ));

    let theme_switch = Switch::new();
    theme_switch.set_active(crate::ui::theme::get_theme() == "dark");
    theme_switch.set_valign(gtk4::Align::Center);
    theme_switch.set_margin_start(15);
    theme_switch.set_margin_end(15);

    let theme_dark_label = Label::new(Some(&t("theme_dark")));
    theme_dark_label.set_markup(&format!("<span size='large'>🌙 {}</span>", t("theme_dark")));

    theme_row.append(&theme_light_label);
    theme_row.append(&theme_switch);
    theme_row.append(&theme_dark_label);
    theme_box.append(&theme_row);

    let theme_status = Label::new(None);
    theme_status.set_halign(gtk4::Align::Center);
    theme_status.set_margin_top(10);
    theme_box.append(&theme_status);

    theme_switch.connect_state_set(glib::clone!(
        #[weak]
        theme_status,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_switch, state| {
            let new_theme = if state { "dark" } else { "light" };
            crate::ui::theme::set_theme(new_theme);

            // Apply theme immediately
            crate::core::debug::debug_log_args(std::format_args!(
                "🎨 [UI_TAB] Theme switch toggled -> {new_theme}"
            ));
            if new_theme == "dark" {
                crate::ui::theme::apply_dark_theme();
            } else {
                crate::ui::theme::apply_light_theme();
            }

            // Save to config file
            if let Some(config_dir) = dirs::config_dir() {
                let app_config_dir = config_dir.join("battery-manager");
                let _ = std::fs::create_dir_all(&app_config_dir);
                let config_file = app_config_dir.join("theme.conf");
                let _ = std::fs::write(config_file, new_theme);
                crate::core::debug::debug_log_args(std::format_args!(
                    "💾 [UI_TAB] Saved theme.conf -> {new_theme}"
                ));
            }

            theme_status.set_markup(&format!(
                "<span size='small'>✓ {}</span>",
                t("theme_applied")
            ));
            theme_status.remove_css_class("color-warning");
            theme_status.remove_css_class("color-danger");
            theme_status.add_css_class("color-success");
            crate::core::debug::debug_log(
                "✅ [UI_TAB] Theme status message updated with color-success class",
            );

            glib::Propagation::Proceed
        }
    ));

    content_box.append(&theme_frame);

    scrolled.set_child(Some(&content_box));
    scrolled
}
