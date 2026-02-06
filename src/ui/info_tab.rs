//! Information tab displaying battery metrics and status
//!
//! Shows charge thresholds, current status, voltage, power consumption,
//! capacity, health, and systemd service status with auto-refresh.

use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

use crate::core::i18n::t;
use crate::core::{BatteryInfo, PowerSupplyInfo};
use crate::ui::components::{
    create_content_box, create_info_label, create_row_grid, create_vertical_spacer, InfoCard,
    UpdatableWidgets,
};

/// Builds the Information tab content
///
/// # Arguments
///
/// * `info` - Battery information
/// * `power_supply` - AC power supply information
///
/// # Returns
///
/// Tuple of (tab Box, `UpdatableWidgets`) for timer updates
#[allow(clippy::too_many_lines)]
pub fn build_info_tab(
    info: &BatteryInfo,
    power_supply: &PowerSupplyInfo,
) -> (Box, UpdatableWidgets) {
    crate::core::debug::debug_log("📋 [INFO_TAB] Building info tab...");
    let content_box = create_content_box(10);

    // === LIGNE 1: Seuils + Charge + Santé ===
    let row1 = create_row_grid();

    // Card Seuils
    let (thresholds_frame, thresholds_box) =
        InfoCard::create(&format!("🎚️ {}", t("card_thresholds")));

    // Grille horizontale pour les seuils
    let thresholds_grid = gtk4::Grid::new();
    thresholds_grid.set_column_spacing(20);
    thresholds_grid.set_column_homogeneous(true);
    thresholds_grid.set_halign(gtk4::Align::Center);

    let mut col = 0;

    // Seuil de début (seulement si supporté)
    let threshold_start_label = info.charge_start_threshold.map_or_else(
        || None,
        |threshold| {
            let col_box = Box::new(Orientation::Vertical, 4);

            let title = Label::new(Some(&t("threshold_start")));
            title.set_halign(gtk4::Align::Center);
            col_box.append(&title);

            let value = Label::new(None);
            value.set_halign(gtk4::Align::Center);
            value.set_markup(&format!(
                "<span size='x-large' weight='bold'>{threshold}%</span>"
            ));
            value.add_css_class("color-primary");
            crate::core::debug::debug_log_args(std::format_args!(
                "🎨 [INFO_TAB] Start threshold label: added color-primary class ({threshold}%)"
            ));
            col_box.append(&value);

            thresholds_grid.attach(&col_box, col, 0, 1, 1);
            col += 1;
            Some(value)
        },
    );

    // Seuil de fin
    let stop_col_box = Box::new(Orientation::Vertical, 4);

    let stop_title = Label::new(Some(&t("threshold_stop")));
    stop_title.set_halign(gtk4::Align::Center);
    stop_col_box.append(&stop_title);

    let threshold_stop_label = Label::new(None);
    threshold_stop_label.set_halign(gtk4::Align::Center);
    threshold_stop_label.set_markup(&format!(
        "<span size='x-large' weight='bold'>{}</span>",
        info.charge_stop_threshold
            .map_or_else(|| "N/A".to_string(), |v| format!("{v}%"))
    ));
    threshold_stop_label.add_css_class("color-success");
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Stop threshold label: added color-success class ({:?})",
        info.charge_stop_threshold
    ));
    stop_col_box.append(&threshold_stop_label);

    thresholds_grid.attach(&stop_col_box, col, 0, 1, 1);
    col += 1;

    // Alarme de décharge (seulement si supportée)
    let alarm_label = info.alarm_percent().map_or_else(
        || None,
        |alarm_pct| {
            let alarm_col_box = Box::new(Orientation::Vertical, 4);

            let alarm_title = Label::new(Some(&t("alarm")));
            alarm_title.set_halign(gtk4::Align::Center);
            alarm_col_box.append(&alarm_title);

            let value = Label::new(None);
            value.set_halign(gtk4::Align::Center);
            value.set_markup(&format!(
                "<span size='x-large' weight='bold'>{alarm_pct:.1}%</span>"
            ));
            value.add_css_class("color-danger");
            crate::core::debug::debug_log_args(std::format_args!(
                "🎨 [INFO_TAB] Alarm label: added color-danger class ({alarm_pct:.1}%)"
            ));
            alarm_col_box.append(&value);

            thresholds_grid.attach(&alarm_col_box, col, 0, 1, 1);
            Some(value)
        },
    );

    thresholds_box.append(&thresholds_grid);

    // Espaceur pour uniformiser la hauteur
    thresholds_box.append(&create_vertical_spacer());

    // Lignes vides pour uniformiser avec les autres cards
    thresholds_box.append(&create_info_label(""));
    thresholds_box.append(&create_info_label(""));
    thresholds_box.append(&create_info_label(""));

    row1.attach(&thresholds_frame, 0, 0, 1, 1);

    // Card Charge
    let (charge_frame, charge_box) = InfoCard::create(&format!("🔋 {}", t("card_charge")));
    charge_box.append(&create_info_label(""));

    let capacity_label = Label::new(None);
    capacity_label.set_halign(gtk4::Align::Center);
    capacity_label.set_markup(&format!(
        "<span size='xx-large' weight='bold'>{}</span><span size='large'>%</span>",
        info.capacity_percent
    ));
    capacity_label.add_css_class("color-primary");
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Capacity label: added color-primary class ({}%)",
        info.capacity_percent
    ));
    charge_box.append(&capacity_label);

    // Espaceur pour pousser les infos secondaires vers le bas
    charge_box.append(&create_vertical_spacer());

    charge_box.append(&create_info_label(""));
    charge_box.append(&create_info_label(""));
    if let Some(time_text) = info.time_remaining_formatted() {
        charge_box.append(&create_info_label(&time_text));
    } else {
        charge_box.append(&create_info_label(""));
    }
    row1.attach(&charge_frame, 1, 0, 1, 1);

    // Card Santé
    let (health_frame, health_box) = InfoCard::create(&format!("❤️ {}", t("card_health")));
    health_box.append(&create_info_label(""));

    let health_label = Label::new(None);
    health_label.set_halign(gtk4::Align::Center);
    health_label.set_markup(&format!(
        "<span size='xx-large' weight='bold'>{:.1}</span><span size='large'>%</span>",
        info.health_percent
    ));
    let health_class = info.get_health_css_class();
    health_label.add_css_class(health_class);
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Health label: added {} class ({:.1}%)",
        health_class,
        info.health_percent
    ));
    health_box.append(&health_label);

    // Espaceur pour pousser les infos secondaires vers le bas
    health_box.append(&create_vertical_spacer());

    health_box.append(&create_info_label(""));
    health_box.append(&create_info_label(&format!(
        "{}: {:.1}%",
        t("wear"),
        info.wear_percent
    )));
    health_box.append(&create_info_label(&format!(
        "{}: {}",
        t("cycles"),
        info.cycle_count
    )));
    row1.attach(&health_frame, 2, 0, 1, 1);

    content_box.append(&row1);

    // === LIGNE 2: Alimentation + État + Batterie ===
    let row2 = create_row_grid();

    // Card Alimentation
    let (power_frame, power_box) = InfoCard::create(&format!("🔌 {}", t("card_power")));
    power_box.append(&create_info_label(""));

    let power_source_value = Label::new(None);
    power_source_value.set_halign(gtk4::Align::Center);
    power_source_value.set_markup(&power_supply.get_power_source_markup());
    power_source_value.add_css_class(power_supply.get_power_source_css_class());
    power_box.append(&power_source_value);

    // Espaceur pour pousser les infos secondaires vers le bas
    power_box.append(&create_vertical_spacer());

    power_box.append(&create_info_label(""));
    power_box.append(&create_info_label(""));
    power_box.append(&create_info_label(&format!(
        "{}: {}",
        t("adapter"),
        power_supply.ac_name
    )));
    row2.attach(&power_frame, 0, 0, 1, 1);

    // Card État
    let (status_frame, status_box) = InfoCard::create(&format!("📊 {}", t("card_status")));
    status_box.append(&create_info_label(""));

    let status_value = Label::new(None);
    status_value.set_halign(gtk4::Align::Center);
    status_value.set_markup(&info.get_status_markup());
    let status_class = info.get_status_css_class();
    status_value.add_css_class(status_class);
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Status label: added {} class ({})",
        status_class,
        info.status
    ));
    status_box.append(&status_value);

    // Espaceur pour pousser les infos secondaires vers le bas
    status_box.append(&create_vertical_spacer());

    status_box.append(&create_info_label(""));
    status_box.append(&create_info_label(""));
    status_box.append(&create_info_label(&format!(
        "{}: {}",
        t("capacity_level"),
        info.capacity_level
    )));
    row2.attach(&status_frame, 1, 0, 1, 1);

    // Card Batterie
    let (battery_frame, battery_box) = InfoCard::create(&format!("🔋 {}", t("card_battery")));
    battery_box.append(&create_info_label(""));

    let battery_main = Label::new(None);
    battery_main.set_halign(gtk4::Align::Center);
    battery_main.set_markup(&format!(
        "<span size='xx-large' weight='bold'>{}</span>",
        info.manufacturer.trim()
    ));
    battery_main.add_css_class("color-primary");
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Battery manufacturer label: added color-primary class ({})",
        info.manufacturer.trim()
    ));
    battery_box.append(&battery_main);

    // Espaceur pour pousser les infos secondaires vers le bas
    battery_box.append(&create_vertical_spacer());

    battery_box.append(&create_info_label(&format!("{}: {}", t("name"), info.name)));
    battery_box.append(&create_info_label(&format!(
        "{}: {}",
        t("model"),
        info.model_name
    )));
    battery_box.append(&create_info_label(&format!(
        "{}: {}",
        t("type"),
        info.technology
    )));
    row2.attach(&battery_frame, 2, 0, 1, 1);

    content_box.append(&row2);

    // === LIGNE 3: Électrique + Capacité + Infos ===
    let row3 = create_row_grid();

    // Card Électrique
    let (electrical_frame, electrical_box) =
        InfoCard::create(&format!("⚡ {}", t("card_electrical")));
    electrical_box.append(&create_info_label(""));

    let power_main = Label::new(None);
    power_main.set_halign(gtk4::Align::Center);
    power_main.set_markup(&format!(
        "<span size='xx-large' weight='bold'>{:.2}</span><span size='large'> W</span>",
        info.power_watts()
    ));
    power_main.add_css_class("color-warning");
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Power label: added color-warning class ({:.2}W)",
        info.power_watts()
    ));
    electrical_box.append(&power_main);

    // Espaceur pour pousser les infos secondaires vers le bas
    electrical_box.append(&create_vertical_spacer());

    let voltage_value = create_info_label(&format!("{}: {:.2} V", t("voltage"), info.voltage_v()));
    electrical_box.append(&voltage_value);
    let current_value = create_info_label(&format!("{}: {} mA", t("current"), info.current_ma()));
    electrical_box.append(&current_value);
    let power_value = create_info_label(&format!("{}: {:.2} W", t("power"), info.power_watts()));
    electrical_box.append(&power_value);
    row3.attach(&electrical_frame, 0, 0, 1, 1);

    // Card Capacité
    let (capacity_frame, capacity_box) = InfoCard::create(&format!("⚡ {}", t("card_capacity")));
    capacity_box.append(&create_info_label(""));

    let capacity_main = Label::new(None);
    capacity_main.set_halign(gtk4::Align::Center);
    capacity_main.set_markup(&format!(
        "<span size='xx-large' weight='bold'>{}</span><span size='large'> mAh</span>",
        info.charge_now_mah()
    ));
    capacity_main.add_css_class("color-primary");
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Capacity (mAh) label: added color-primary class ({}mAh)",
        info.charge_now_mah()
    ));
    capacity_box.append(&capacity_main);

    // Espaceur pour pousser les infos secondaires vers le bas
    capacity_box.append(&create_vertical_spacer());

    let charge_now_value = create_info_label(&format!(
        "{}: {} mAh",
        t("current_capacity"),
        info.charge_now_mah()
    ));
    capacity_box.append(&charge_now_value);
    capacity_box.append(&create_info_label(&format!(
        "{}: {} mAh",
        t("full_capacity"),
        info.charge_full_mah()
    )));
    capacity_box.append(&create_info_label(&format!(
        "{}: {} mAh",
        t("design_capacity"),
        info.charge_full_design_mah()
    )));
    row3.attach(&capacity_frame, 1, 0, 1, 1);

    // Card Service
    let (service_frame, service_box) = InfoCard::create(&format!("🔄 {}", t("card_service")));
    service_box.append(&create_info_label(""));

    let service_label = Label::new(None);
    service_label.set_halign(gtk4::Align::Center);
    service_label.set_markup(&info.service_status_markup());
    let service_class = info.service_status_css_class();
    service_label.add_css_class(service_class);
    crate::core::debug::debug_log_args(std::format_args!(
        "🎨 [INFO_TAB] Service label: added {} class (active={})",
        service_class,
        info.service_active
    ));
    service_box.append(&service_label);

    // Espaceur pour pousser les infos secondaires vers le bas
    service_box.append(&create_vertical_spacer());

    service_box.append(&create_info_label(""));
    service_box.append(&create_info_label(""));
    service_box.append(&create_info_label(""));

    row3.attach(&service_frame, 2, 0, 1, 1);

    content_box.append(&row3);

    // Create updatable widgets structure
    let updatable = UpdatableWidgets {
        power_source_value,
        status_value,
        capacity_label,
        health_label,
        voltage_value,
        current_value,
        power_value,
        charge_now_value,
        threshold_start_label,
        threshold_stop_label,
        alarm_label,
        service_label,
    };

    (content_box, updatable)
}
