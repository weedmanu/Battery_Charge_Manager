//! Settings tab for configuring battery charge thresholds
//!
//! Allows users to adjust start/stop charge thresholds, enable/disable
//! systemd service, and view hardware support information.

use gtk4::prelude::*;
use gtk4::{Adjustment, Box, Button, Label, Orientation, ScrolledWindow, SpinButton, Switch};
use std::fmt::Write;
use std::path::Path;
use std::process::Command;

use crate::core::i18n::t;
use crate::core::{BatteryInfo, VendorInfo};
use crate::ui::components::InfoCard;

fn service_unit_exists() -> bool {
    [
        "/etc/systemd/system/battery-manager.service",
        "/usr/lib/systemd/system/battery-manager.service",
        "/lib/systemd/system/battery-manager.service",
    ]
    .into_iter()
    .any(|p| Path::new(p).is_file())
}

/// Truncates a string for logging purposes
fn truncate_for_log(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let mut out = s.chars().take(max_chars).collect::<String>();
    out.push('…');
    out
}

/// Computed alarm value from battery `charge_full`/`energy_full` and percentage
fn compute_alarm_value(battery_name: &str, alarm_pct: f32) -> String {
    let charge_full_str = std::fs::read_to_string(format!(
        "/sys/class/power_supply/{battery_name}/charge_full"
    ))
    .or_else(|charge_err| {
        crate::core::debug::debug_log_args(std::format_args!(
            "⚠️ [SETTINGS_TAB] charge_full read failed, trying energy_full: {charge_err}"
        ));
        std::fs::read_to_string(format!(
            "/sys/class/power_supply/{battery_name}/energy_full"
        ))
    });

    match charge_full_str {
        Ok(charge_full) => match charge_full.trim().parse::<u64>() {
            Ok(full_value) => {
                #[allow(
                    clippy::cast_precision_loss,
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss
                )]
                let alarm_value = (full_value as f32 * (alarm_pct / 100.0)) as u64;
                crate::core::debug::debug_log_args(std::format_args!(
                    "🧮 [SETTINGS_TAB] Alarm computed from full_value={full_value}: alarm_value={alarm_value}"
                ));
                alarm_value.to_string()
            }
            Err(parse_err) => {
                crate::core::debug::debug_log_args(std::format_args!(
                    "⚠️ [SETTINGS_TAB] Failed to parse charge_full/energy_full value: {parse_err} (raw='{}')",
                    truncate_for_log(charge_full.trim(), 80)
                ));
                "0".to_string()
            }
        },
        Err(read_err) => {
            crate::core::debug::debug_log_args(std::format_args!(
                "⚠️ [SETTINGS_TAB] Failed to read charge_full/energy_full: {read_err}; falling back to alarm_value=0"
            ));
            "0".to_string()
        }
    }
}

/// Builds the shell script to apply thresholds, alarm, and service configuration
fn build_apply_script(
    battery_name: &str,
    start: u8,
    stop: u8,
    has_start: bool,
    alarm_value_str: &str,
    enable_service: bool,
) -> String {
    let base_path = format!("/sys/class/power_supply/{battery_name}");
    let alarm_path = format!("{base_path}/alarm");
    let start_paths = [
        format!("{base_path}/charge_control_start_threshold"),
        format!("{base_path}/charge_start_threshold"),
    ];
    let stop_paths = [
        format!("{base_path}/charge_control_end_threshold"),
        format!("{base_path}/charge_stop_threshold"),
        format!("{base_path}/charge_end_threshold"),
    ];

    crate::core::debug::debug_log_args(std::format_args!(
        "🗂️ [SETTINGS_TAB] Sysfs paths: alarm_path='{alarm_path}' exists={}, start_paths_exist=[{}, {}], stop_paths_exist=[{}, {}, {}]",
        Path::new(&alarm_path).is_file(),
        Path::new(&start_paths[0]).is_file(),
        Path::new(&start_paths[1]).is_file(),
        Path::new(&stop_paths[0]).is_file(),
        Path::new(&stop_paths[1]).is_file(),
        Path::new(&stop_paths[2]).is_file(),
    ));

    let mut script = String::new();

    // Create config directory
    script.push_str("mkdir -p /etc/battery-manager; ");

    // Write thresholds (values are pre-validated numeric strings)
    for path in &start_paths {
        let _ = write!(&mut script, "[ -f {path} ] && echo {start} > {path}; ");
    }
    for path in &stop_paths {
        let _ = write!(&mut script, "[ -f {path} ] && echo {stop} > {path}; ");
    }

    // Write alarm
    let _ = write!(
        &mut script,
        "[ -f {alarm_path} ] && echo {alarm_value_str} > {alarm_path}; "
    );

    // Save config (START_THRESHOLD only if supported)
    let config_content = if has_start {
        format!("START_THRESHOLD={start}\\nSTOP_THRESHOLD={stop}\\n")
    } else {
        format!("STOP_THRESHOLD={stop}\\n")
    };
    let _ = write!(
        &mut script,
        "echo '{config_content}' > /etc/battery-manager/{battery_name}.conf; "
    );

    // Manage service
    if enable_service {
        script.push_str("systemctl enable battery-manager.service; ");
        script.push_str("systemctl start battery-manager.service; ");
    } else {
        script.push_str("systemctl disable battery-manager.service 2>/dev/null || true; ");
        script.push_str("systemctl stop battery-manager.service 2>/dev/null || true; ");
    }

    crate::core::debug::debug_log_args(std::format_args!(
        "🔧 [SETTINGS_TAB] Prepared script: bytes={}, service_enable={enable_service}",
        script.len()
    ));

    script
}

/// Result of executing settings via pkexec
enum ApplyResult {
    /// Settings applied successfully
    Success,
    /// pkexec execution failed
    Failed(String),
    /// pkexec not installed
    NoPkexec,
}

/// Executes the apply script via pkexec and returns the result
fn execute_with_pkexec(script: &str) -> ApplyResult {
    let pkexec_check = Command::new("which").arg("pkexec").output();

    match pkexec_check {
        Ok(result) if result.status.success() => {
            crate::core::debug::debug_log(
                "🔐 [SETTINGS_TAB] pkexec found, executing script via pkexec",
            );
            let output = Command::new("pkexec")
                .arg("sh")
                .arg("-c")
                .arg(script)
                .output();

            match output {
                Ok(result) if result.status.success() => {
                    crate::core::debug::debug_log("✅ [SETTINGS_TAB] pkexec execution succeeded");
                    ApplyResult::Success
                }
                Ok(result) => {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    let code = result.status.code();
                    let stderr_preview = truncate_for_log(stderr.trim(), 400);
                    let stdout_preview = truncate_for_log(stdout.trim(), 400);

                    let ui_error = if !stderr.trim().is_empty() {
                        stderr_preview.clone()
                    } else if !stdout.trim().is_empty() {
                        stdout_preview.clone()
                    } else {
                        format!("pkexec returned non-zero status: {code:?}")
                    };

                    crate::core::debug::debug_log_args(std::format_args!(
                        "❌ [SETTINGS_TAB] Script execution failed: code={code:?} stdout='{stdout_preview}' stderr='{stderr_preview}'"
                    ));
                    ApplyResult::Failed(ui_error)
                }
                Err(err) => {
                    crate::core::debug::debug_log_args(std::format_args!(
                        "❌ [SETTINGS_TAB] Execution error: {err}"
                    ));
                    ApplyResult::Failed(format!("{}: {err}", t("error_execution")))
                }
            }
        }
        _ => {
            crate::core::debug::debug_log(
                "❌ [SETTINGS_TAB] pkexec not found (which pkexec failed or returned non-zero)",
            );
            ApplyResult::NoPkexec
        }
    }
}

/// Updates the status message label with appropriate color class
fn set_status_class(label: &Label, class: &str) {
    label.remove_css_class("color-success");
    label.remove_css_class("color-warning");
    label.remove_css_class("color-danger");
    label.add_css_class(class);
}

/// Creates vendor information card
fn create_vendor_card(vendor_info: &VendorInfo) -> gtk4::Frame {
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

    vendor_frame
}

/// Creates threshold spinbutton row
fn create_threshold_row(
    label_text: &str,
    default_value: u8,
    min: f64,
    max: f64,
) -> (Box, SpinButton) {
    let row = Box::new(Orientation::Horizontal, 10);
    row.set_homogeneous(true);

    let label = Label::new(Some(label_text));
    label.set_halign(gtk4::Align::Start);
    label.set_markup(&format!("<span weight='bold'>{label_text}</span>"));

    let adj = Adjustment::new(f64::from(default_value), min, max, 1.0, 5.0, 0.0);
    let spin = SpinButton::new(Some(&adj), 1.0, 0);
    spin.set_halign(gtk4::Align::End);

    row.append(&label);
    row.append(&spin);

    (row, spin)
}

/// Builds the Settings tab content
///
/// # Arguments
///
/// * `battery_info` - Current battery information
/// * `current_battery` - Name of active battery (e.g., "BAT0")
///
/// # Returns
///
/// `ScrolledWindow` containing settings controls
#[allow(clippy::too_many_lines)]
pub fn build_settings_tab(battery_info: &BatteryInfo, current_battery: &str) -> ScrolledWindow {
    crate::core::debug::debug_log_args(std::format_args!(
        "⚙️ [SETTINGS_TAB] Building settings tab for {current_battery}..."
    ));

    let unit_exists = service_unit_exists();
    crate::core::debug::debug_log_args(std::format_args!(
        "🧩 [SETTINGS_TAB] Service unit present: {unit_exists} (service_active={})",
        battery_info.service_active
    ));
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let content_box = Box::new(Orientation::Vertical, 6);
    content_box.set_margin_top(8);
    content_box.set_margin_bottom(8);
    content_box.set_margin_start(10);
    content_box.set_margin_end(10);

    // === Card Informations Fabricant ===
    let vendor_info = VendorInfo::detect();
    let vendor_frame = create_vendor_card(&vendor_info);
    content_box.append(&vendor_frame);

    // === Card Seuils de charge ===
    let (settings_frame, settings_box) =
        InfoCard::create(&format!("⚙️ {}", t("card_threshold_settings")));
    settings_box.set_spacing(8);

    // Seuil début (seulement si supporté)
    let start_spin = battery_info.charge_start_threshold.map(|threshold| {
        let (start_row, spin) =
            create_threshold_row(&t("threshold_start_pct"), threshold, 0.0, 99.0);
        settings_box.append(&start_row);
        spin
    });

    // Seuil fin
    let (stop_row, stop_spin) = create_threshold_row(
        &t("threshold_stop_pct"),
        battery_info.charge_stop_threshold.unwrap_or(80),
        1.0,
        100.0,
    );
    settings_box.append(&stop_row);

    // Alarme de décharge
    let alarm_value = battery_info.alarm_percent().unwrap_or(10.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (alarm_row, alarm_spin) =
        create_threshold_row(&t("alarm_threshold"), alarm_value as u8, 1.0, 100.0);
    // Override decimal places for alarm
    alarm_spin.set_digits(1);
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

    service_switch.connect_state_set(|_, is_active| {
        crate::core::debug::debug_log_args(std::format_args!(
            "🔁 [SETTINGS_TAB] Service switch toggled: active={is_active}"
        ));
        glib::Propagation::Proceed
    });

    service_row.append(&service_label);
    service_row.append(&service_switch);
    service_box.append(&service_row);

    // Note d'information avec fond coloré
    let note_frame = gtk4::Frame::new(None);
    note_frame.set_margin_top(5);
    note_frame.add_css_class("info-note");

    let note_box = Box::new(Orientation::Vertical, 4);
    note_box.set_margin_top(6);
    note_box.set_margin_bottom(6);
    note_box.set_margin_start(12);
    note_box.set_margin_end(12);

    let note1 = Label::new(None);
    note1.set_halign(gtk4::Align::Start);
    note1.set_markup(&format!("<span size='small'>{}</span>", t("note_enabled")));
    note_box.append(&note1);

    let note2 = Label::new(None);
    note2.set_halign(gtk4::Align::Start);
    note2.set_markup(&format!("<span size='small'>{}</span>", t("note_disabled")));
    note_box.append(&note2);

    let note3 = Label::new(None);
    note3.set_halign(gtk4::Align::Start);
    note3.set_markup(&format!(
        "<span size='small'>{}</span>",
        t("note_apply_required")
    ));
    note_box.append(&note3);

    note_frame.set_child(Some(&note_box));
    service_box.append(&note_frame);

    content_box.append(&service_frame);

    // Message de statut (en dehors du frame)
    let status_message = Label::new(None);
    status_message.set_halign(gtk4::Align::Center);
    status_message.set_margin_top(10);
    content_box.append(&status_message);

    // Single button to apply all modifications (centered outside frame)
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
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let start = start_spin.as_ref().map_or(0, |s| s.value() as u8);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let stop = stop_spin.value() as u8;
            #[allow(clippy::cast_possible_truncation)]
            let alarm_pct = alarm_spin.value() as f32;
            let enable_service = service_switch.is_active();
            let has_start = start_spin.is_some();

            crate::core::debug::debug_log_args(std::format_args!(
                "🧾 [SETTINGS_TAB] Apply clicked: start_supported={has_start}, start={start}, stop={stop}, alarm_pct={alarm_pct:.1}, service_enable={enable_service}"
            ));

            if !enable_service {
                crate::core::debug::debug_log(
                    "⚠️ [SETTINGS_TAB] Service disabled: thresholds apply now, but may not persist after reboot",
                );
            }

            // Validation
            if has_start && start >= stop {
                status_message.set_markup(&format!(
                    "<span>{}</span>",
                    t("error_start_greater_stop")
                ));
                set_status_class(&status_message, "color-danger");
                crate::core::debug::debug_log_args(std::format_args!(
                    "❌ [SETTINGS_TAB] Validation error: start ({start}) >= stop ({stop})"
                ));
                return;
            }

            // Compute alarm value
            let alarm_value_str = compute_alarm_value(&current_battery_clone, alarm_pct);

            // Validate numeric inputs
            if !alarm_value_str.chars().all(|c| c.is_ascii_digit()) {
                status_message.set_markup(&format!("<span>{}: Invalid numeric values</span>", t("error")));
                set_status_class(&status_message, "color-danger");
                crate::core::debug::debug_log_args(std::format_args!(
                    "❌ [SETTINGS_TAB] Numeric validation failed: alarm_value_str='{}'",
                    truncate_for_log(&alarm_value_str, 80)
                ));
                return;
            }

            // Build and execute script
            let script = build_apply_script(
                &current_battery_clone,
                start, stop, has_start,
                &alarm_value_str,
                enable_service,
            );

            match execute_with_pkexec(&script) {
                ApplyResult::Success => {
                    let service_status = if enable_service { t("enabled") } else { t("disabled") };
                    let threshold_msg = if has_start {
                        format!("{start}%-{stop}%")
                    } else {
                        format!("{stop}%")
                    };
                    let persistence_note = if enable_service {
                        String::new()
                    } else {
                        format!("\n<span size='small'>{}</span>", t("warning_not_persistent"))
                    };
                    status_message.set_markup(&format!(
                        "<span>✓ {}: {}, {}: {:.1}%, {}: {}{}</span>",
                        t("settings_applied"), threshold_msg, t("alarm"), alarm_pct,
                        t("service"), service_status, persistence_note
                    ));
                    set_status_class(&status_message, "color-success");
                    crate::core::debug::debug_log_args(std::format_args!(
                        "✅ [SETTINGS_TAB] Settings applied successfully: {threshold_msg}, alarm={alarm_pct:.1}%, service={service_status}"
                    ));
                }
                ApplyResult::Failed(error_msg) => {
                    status_message.set_markup(&format!("<span>{}: {}</span>", t("error"), error_msg));
                    set_status_class(&status_message, "color-danger");
                }
                ApplyResult::NoPkexec => {
                    status_message.set_markup(&format!(
                        "<span>{}: pkexec not installed. Install policykit-1 or polkit.</span>",
                        t("error")
                    ));
                    set_status_class(&status_message, "color-danger");
                }
            }
            }
        ),
    );

    content_box.append(&apply_button);

    scrolled.set_child(Some(&content_box));
    scrolled
}
