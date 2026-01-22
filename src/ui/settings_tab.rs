//! Settings tab for configuring battery charge thresholds
//!
//! Allows users to adjust start/stop charge thresholds, enable/disable
//! systemd service, and view hardware support information.

use gtk4::prelude::*;
use gtk4::{Adjustment, Box, Button, Label, Orientation, ScrolledWindow, SpinButton, Switch};
use std::process::Command;

use crate::core::i18n::t;
use crate::core::{BatteryInfo, VendorInfo};
use crate::ui::components::InfoCard;

/// Builds the Settings tab content
///
/// # Arguments
///
/// * `battery_info` - Current battery information
/// * `current_battery` - Name of active battery (e.g., "BAT0")
///
/// # Returns
///
/// ScrolledWindow containing settings controls
pub fn build_settings_tab(battery_info: &BatteryInfo, current_battery: &str) -> ScrolledWindow {
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let content_box = Box::new(Orientation::Vertical, 6);
    content_box.set_margin_top(8);
    content_box.set_margin_bottom(8);
    content_box.set_margin_start(10);
    content_box.set_margin_end(10);

    // === Card Informations Fabricant ===
    let vendor_info = VendorInfo::detect();
    let (vendor_frame, vendor_box) = InfoCard::create(&format!("🏭 {}", t("card_system_info")));
    vendor_box.set_spacing(5);

    let vendor_label = Label::new(None);
    vendor_label.set_halign(gtk4::Align::Start);
    vendor_label.set_markup(&format!(
        "<span weight='bold'>{}:</span> {} | <span weight='bold'>{}:</span> {}",
        t("manufacturer"),
        vendor_info.manufacturer,
        t("model"),
        vendor_info.product_name
    ));
    vendor_box.append(&vendor_label);

    let support_label = Label::new(None);
    support_label.set_halign(gtk4::Align::Start);
    let start_support = if vendor_info.supports_start_threshold {
        "✅"
    } else {
        "❌"
    };
    let stop_support = if vendor_info.supports_stop_threshold {
        "✅"
    } else {
        "❌"
    };
    support_label.set_markup(&format!(
        "<span size='small'>{}: {} | {}: {}</span>",
        t("threshold_start"),
        start_support,
        t("threshold_stop"),
        stop_support
    ));
    vendor_box.append(&support_label);

    content_box.append(&vendor_frame);

    // === Card Seuils de charge ===
    let (settings_frame, settings_box) =
        InfoCard::create(&format!("⚙️ {}", t("card_threshold_settings")));
    settings_box.set_spacing(8);

    // Seuil début (seulement si supporté)
    let start_spin = if battery_info.charge_start_threshold.is_some() {
        let start_row = Box::new(Orientation::Horizontal, 10);
        start_row.set_homogeneous(true);

        let start_label = Label::new(Some(&t("threshold_start_pct")));
        start_label.set_halign(gtk4::Align::Start);
        start_label.set_markup(&format!(
            "<span weight='bold'>{}</span>",
            t("threshold_start_pct")
        ));

        let start_adj = Adjustment::new(
            battery_info.charge_start_threshold.unwrap_or(75) as f64,
            0.0,
            99.0,
            1.0,
            5.0,
            0.0,
        );
        let spin = SpinButton::new(Some(&start_adj), 1.0, 0);
        spin.set_halign(gtk4::Align::End);

        start_row.append(&start_label);
        start_row.append(&spin);
        settings_box.append(&start_row);
        Some(spin)
    } else {
        None
    };

    // Seuil fin
    let stop_row = Box::new(Orientation::Horizontal, 10);
    stop_row.set_homogeneous(true);

    let stop_label = Label::new(Some(&t("threshold_stop_pct")));
    stop_label.set_halign(gtk4::Align::Start);
    stop_label.set_markup(&format!(
        "<span weight='bold'>{}</span>",
        t("threshold_stop_pct")
    ));

    let stop_adj = Adjustment::new(
        battery_info.charge_stop_threshold.unwrap_or(80) as f64,
        1.0,
        100.0,
        1.0,
        5.0,
        0.0,
    );
    let stop_spin = SpinButton::new(Some(&stop_adj), 1.0, 0);
    stop_spin.set_halign(gtk4::Align::End);

    stop_row.append(&stop_label);
    stop_row.append(&stop_spin);
    settings_box.append(&stop_row);

    // Alarme de décharge
    let alarm_row = Box::new(Orientation::Horizontal, 10);
    alarm_row.set_homogeneous(true);

    let alarm_label = Label::new(Some("Alarme de décharge (%)"));
    alarm_label.set_halign(gtk4::Align::Start);
    alarm_label.set_markup("<span weight='bold'>Alarme de décharge (%)</span>");

    let alarm_value = battery_info.alarm_percent().unwrap_or(10.0);
    let alarm_adj = Adjustment::new(alarm_value as f64, 1.0, 100.0, 1.0, 5.0, 0.0);
    let alarm_spin = SpinButton::new(Some(&alarm_adj), 1.0, 1);
    alarm_spin.set_halign(gtk4::Align::End);

    alarm_row.append(&alarm_label);
    alarm_row.append(&alarm_spin);
    settings_box.append(&alarm_row);

    content_box.append(&settings_frame);

    // === Card Service ===
    let (service_frame, service_box) =
        InfoCard::create(&format!("🔄 {}", t("card_service_manager")));
    service_box.set_spacing(8);

    let service_row = Box::new(Orientation::Horizontal, 10);

    let service_label = Label::new(Some(&t("enable_service")));
    service_label.set_halign(gtk4::Align::Start);
    service_label.set_hexpand(true);
    service_label.set_markup(&format!(
        "<span weight='bold'>{}</span>",
        t("enable_systemd_service")
    ));

    let service_switch = Switch::new();
    service_switch.set_active(battery_info.service_active);
    service_switch.set_valign(gtk4::Align::Center);
    service_switch.set_halign(gtk4::Align::End);

    service_row.append(&service_label);
    service_row.append(&service_switch);
    service_box.append(&service_row);

    // Note d'information avec fond coloré
    let note_frame = gtk4::Frame::new(None);
    note_frame.set_margin_top(5);

    let note_box = Box::new(Orientation::Vertical, 4);
    note_box.set_margin_top(6);
    note_box.set_margin_bottom(6);
    note_box.set_margin_start(12);
    note_box.set_margin_end(12);

    // Ajouter un style CSS pour le fond coloré
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data("frame { background-color: #E3F2FD; border-radius: 6px; }");
    note_frame
        .style_context()
        .add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let note1 = Label::new(None);
    note1.set_halign(gtk4::Align::Start);
    note1.set_markup(&format!("<span size='small'>{}</span>", t("note_enabled")));
    note_box.append(&note1);

    let note2 = Label::new(None);
    note2.set_halign(gtk4::Align::Start);
    note2.set_markup(&format!("<span size='small'>{}</span>", t("note_disabled")));
    note_box.append(&note2);

    note_frame.set_child(Some(&note_box));
    service_box.append(&note_frame);

    content_box.append(&service_frame);

    // Message de statut (en dehors du frame)
    let status_message = Label::new(None);
    status_message.set_halign(gtk4::Align::Center);
    status_message.set_margin_top(10);
    content_box.append(&status_message);

    // Bouton unique pour appliquer toutes les modifications (centré en dehors du frame)
    let current_battery_clone = current_battery.to_string();
    let apply_button = Button::with_label(&t("apply_all_settings"));
    apply_button.set_margin_top(10);
    apply_button.set_halign(gtk4::Align::Center);

    // Style CSS pour le bouton
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        "button {
            background: linear-gradient(to bottom, #4CAF50, #45a049);
            color: white;
            font-weight: bold;
            font-size: 14px;
            padding: 12px 24px;
            border-radius: 8px;
            border: 1px solid #3d8b40;
            box-shadow: 0 2px 5px rgba(0,0,0,0.2);
        }
        button:hover {
            background: linear-gradient(to bottom, #45a049, #3d8b40);
        }
        button:active {
            box-shadow: 0 1px 2px rgba(0,0,0,0.2);
        }",
    );
    apply_button
        .style_context()
        .add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    apply_button.connect_clicked(
        glib::clone!(
            #[weak]
            stop_spin,
            #[weak]
            alarm_spin,
            #[weak]
            service_switch,
            #[weak]
            status_message,
            move |_| {
            let start = start_spin.as_ref().map(|s| s.value() as u8).unwrap_or(0);
            let stop = stop_spin.value() as u8;
            let alarm_pct = alarm_spin.value() as f32;
            let enable_service = service_switch.is_active();

            // Validation
            if start_spin.as_ref().is_some()
                && start >= stop {
                    status_message.set_markup(&format!(
                        "<span color='red'>{}</span>",
                        t("error_start_greater_stop")
                    ));
                    return;
                }

            // Calculer la valeur d'alarme
            let alarm_value_str = if let Ok(charge_full) = std::fs::read_to_string(
                format!("/sys/class/power_supply/{}/charge_full", current_battery_clone)
            ).or_else(|_| std::fs::read_to_string(
                format!("/sys/class/power_supply/{}/energy_full", current_battery_clone)
            )) {
                if let Ok(full_value) = charge_full.trim().parse::<u64>() {
                    let alarm_value = (full_value as f32 * (alarm_pct / 100.0)) as u64;
                    alarm_value.to_string()
                } else {
                    "0".to_string()
                }
            } else {
                "0".to_string()
            };

            // Construire le script shell qui fait tout en une seule fois
            let alarm_path = format!("/sys/class/power_supply/{}/alarm", current_battery_clone);

            // Chemins possibles pour les seuils
            let base_path = format!("/sys/class/power_supply/{}", current_battery_clone);
            let start_paths = vec![
                format!("{}/charge_control_start_threshold", base_path),
                format!("{}/charge_start_threshold", base_path),
            ];
            let stop_paths = vec![
                format!("{}/charge_control_end_threshold", base_path),
                format!("{}/charge_stop_threshold", base_path),
                format!("{}/charge_end_threshold", base_path),
            ];

            let mut script = String::new();

            // Créer le répertoire de config
            script.push_str("mkdir -p /etc/battery-manager; ");

            // Écrire les seuils
            for path in &start_paths {
                script.push_str(&format!("[ -f {} ] && echo {} > {}; ", path, start, path));
            }
            for path in &stop_paths {
                script.push_str(&format!("[ -f {} ] && echo {} > {}; ", path, stop, path));
            }

            // Écrire l'alarme
            script.push_str(&format!("[ -f {} ] && echo {} > {}; ", alarm_path, alarm_value_str, alarm_path));

            // Sauvegarder la config (START_THRESHOLD seulement si supporté)
            let config_content = if start_spin.is_some() {
                format!("START_THRESHOLD={}\\nSTOP_THRESHOLD={}\\n", start, stop)
            } else {
                format!("STOP_THRESHOLD={}\\n", stop)
            };
            script.push_str(&format!("echo '{}' > /etc/battery-manager/{}.conf; ",
                config_content, current_battery_clone));

            // Gérer le service
            if enable_service {
                script.push_str("systemctl enable battery-manager.service; ");
                script.push_str("systemctl start battery-manager.service; ");
            } else {
                script.push_str("systemctl disable battery-manager.service; ");
                script.push_str("systemctl stop battery-manager.service; ");
            }

            // Exécuter tout avec un seul appel pkexec
            // D'abord vérifier que pkexec est disponible
            let pkexec_check = Command::new("which")
                .arg("pkexec")
                .output();

            match pkexec_check {
                Ok(result) if result.status.success() => {
                    // pkexec existe, on peut continuer
                    let output = Command::new("pkexec")
                        .arg("sh")
                        .arg("-c")
                        .arg(&script)
                        .output();

                    match output {
                        Ok(result) if result.status.success() => {
                            let service_status = if enable_service { t("enabled") } else { t("disabled") };
                            let threshold_msg = if start_spin.is_some() {
                                format!("{}%-{}%", start, stop)
                            } else {
                                format!("{}%", stop)
                            };
                            status_message.set_markup(&format!(
                                "<span color='green'>✓ {}: {}, {}: {:.1}%, {}: {}</span>",
                                t("settings_applied"), threshold_msg, t("alarm"), alarm_pct, t("service"), service_status
                            ));
                        }
                        Ok(result) => {
                            let error = String::from_utf8_lossy(&result.stderr);
                            status_message.set_markup(&format!("<span color='red'>{}: {}</span>", t("error"), error));
                        }
                        Err(err) => {
                            status_message.set_markup(&format!("<span color='red'>{}: {}</span>", t("error_execution"), err));
                        }
                    }
                }
                _ => {
                    status_message.set_markup(&format!("<span color='red'>{}: pkexec n'est pas installé. Installez policykit-1 ou polkit.</span>", t("error")));
                }
            }
            }
        ),
    );

    content_box.append(&apply_button);

    scrolled.set_child(Some(&content_box));
    scrolled
}
