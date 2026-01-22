use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};

use crate::core::{BatteryInfo, PowerSupplyInfo};
use crate::ui::components::{
    create_content_box, create_info_label, create_row_grid, create_vertical_spacer, InfoCard,
    UpdatableWidgets,
};

/// Construit l'onglet Informations
pub fn build_info_tab(
    info: &BatteryInfo,
    power_supply: &PowerSupplyInfo,
) -> (Box, UpdatableWidgets) {
    let content_box = create_content_box(10);

    // === LIGNE 1: Seuils + Charge + Santé ===
    let row1 = create_row_grid();

    // Card Seuils
    let (thresholds_frame, thresholds_box) = InfoCard::create("🎚️ Seuils");

    // Grille horizontale pour les seuils
    let thresholds_grid = gtk4::Grid::new();
    thresholds_grid.set_column_spacing(20);
    thresholds_grid.set_column_homogeneous(true);
    thresholds_grid.set_halign(gtk4::Align::Center);

    let mut col = 0;

    // Seuil de début (seulement si supporté)
    let threshold_start_label = if let Some(threshold) = info.charge_start_threshold {
        let col_box = Box::new(Orientation::Vertical, 4);

        let title = Label::new(Some("Début de charge"));
        title.set_halign(gtk4::Align::Center);
        col_box.append(&title);

        let value = Label::new(None);
        value.set_halign(gtk4::Align::Center);
        value.set_markup(&format!(
            "<span size='x-large' weight='bold' color='#2196F3'>{}%</span>",
            threshold
        ));
        col_box.append(&value);

        thresholds_grid.attach(&col_box, col, 0, 1, 1);
        col += 1;
        Some(value)
    } else {
        None
    };

    // Seuil de fin
    let stop_col_box = Box::new(Orientation::Vertical, 4);

    let stop_title = Label::new(Some("Fin de charge"));
    stop_title.set_halign(gtk4::Align::Center);
    stop_col_box.append(&stop_title);

    let threshold_stop_label = Label::new(None);
    threshold_stop_label.set_halign(gtk4::Align::Center);
    threshold_stop_label.set_markup(&format!(
        "<span size='x-large' weight='bold' color='#4CAF50'>{}</span>",
        info.charge_stop_threshold
            .map(|v| format!("{}%", v))
            .unwrap_or_else(|| "N/A".to_string())
    ));
    stop_col_box.append(&threshold_stop_label);

    thresholds_grid.attach(&stop_col_box, col, 0, 1, 1);
    col += 1;

    // Alarme de décharge (seulement si supportée)
    let alarm_label = if let Some(alarm_pct) = info.alarm_percent() {
        let alarm_col_box = Box::new(Orientation::Vertical, 4);

        let alarm_title = Label::new(Some("Alarme"));
        alarm_title.set_halign(gtk4::Align::Center);
        alarm_col_box.append(&alarm_title);

        let value = Label::new(None);
        value.set_halign(gtk4::Align::Center);
        value.set_markup(&format!(
            "<span size='x-large' weight='bold' color='#FF5722'>{:.1}%</span>",
            alarm_pct
        ));
        alarm_col_box.append(&value);

        thresholds_grid.attach(&alarm_col_box, col, 0, 1, 1);
        Some(value)
    } else {
        None
    };

    thresholds_box.append(&thresholds_grid);

    // Espaceur pour uniformiser la hauteur
    thresholds_box.append(&create_vertical_spacer());

    // Lignes vides pour uniformiser avec les autres cards
    thresholds_box.append(&create_info_label(""));
    thresholds_box.append(&create_info_label(""));
    thresholds_box.append(&create_info_label(""));

    row1.attach(&thresholds_frame, 0, 0, 1, 1);

    // Card Charge
    let (charge_frame, charge_box) = InfoCard::create("🔋 Charge");
    charge_box.append(&create_info_label(""));

    let capacity_label = Label::new(None);
    capacity_label.set_halign(gtk4::Align::Center);
    capacity_label.set_markup(&format!(
        "<span size='xx-large' weight='bold' color='#2196F3'>{}</span><span size='large'>%</span>",
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
    let (health_frame, health_box) = InfoCard::create("❤️ Santé");
    health_box.append(&create_info_label(""));

    let health_label = Label::new(None);
    health_label.set_halign(gtk4::Align::Center);
    let health_color = if info.health_percent >= 80.0 {
        "green"
    } else if info.health_percent >= 50.0 {
        "orange"
    } else {
        "red"
    };
    health_label.set_markup(&format!(
        "<span size='xx-large' weight='bold' color='{}'>{:.1}</span><span size='large'>%</span>",
        health_color, info.health_percent
    ));
    health_box.append(&health_label);

    // Espaceur pour pousser les infos secondaires vers le bas
    health_box.append(&create_vertical_spacer());

    health_box.append(&create_info_label(""));
    health_box.append(&create_info_label(&format!(
        "Usure: {:.1}%",
        info.wear_percent
    )));
    health_box.append(&create_info_label(&format!("Cycles: {}", info.cycle_count)));
    row1.attach(&health_frame, 2, 0, 1, 1);

    content_box.append(&row1);

    // === LIGNE 2: Alimentation + État + Batterie ===
    let row2 = create_row_grid();

    // Card Alimentation
    let (power_frame, power_box) = InfoCard::create("🔌 Alimentation");
    power_box.append(&create_info_label(""));

    let power_source_value = Label::new(None);
    power_source_value.set_halign(gtk4::Align::Center);
    power_source_value.set_markup(power_supply.get_power_source_markup());
    power_box.append(&power_source_value);

    // Espaceur pour pousser les infos secondaires vers le bas
    power_box.append(&create_vertical_spacer());

    power_box.append(&create_info_label(""));
    power_box.append(&create_info_label(""));
    power_box.append(&create_info_label(&format!(
        "Adaptateur: {}",
        power_supply.ac_name
    )));
    row2.attach(&power_frame, 0, 0, 1, 1);

    // Card État
    let (status_frame, status_box) = InfoCard::create("📊 État");
    status_box.append(&create_info_label(""));

    let status_value = Label::new(None);
    status_value.set_halign(gtk4::Align::Center);
    status_value.set_markup(&info.get_status_markup());
    status_box.append(&status_value);

    // Espaceur pour pousser les infos secondaires vers le bas
    status_box.append(&create_vertical_spacer());

    status_box.append(&create_info_label(""));
    status_box.append(&create_info_label(""));
    status_box.append(&create_info_label(&format!(
        "Niveau: {}",
        info.capacity_level
    )));
    row2.attach(&status_frame, 1, 0, 1, 1);

    // Card Batterie
    let (battery_frame, battery_box) = InfoCard::create("🔋 Batterie");
    battery_box.append(&create_info_label(""));

    let battery_main = Label::new(None);
    battery_main.set_halign(gtk4::Align::Center);
    battery_main.set_markup(&format!(
        "<span size='xx-large' weight='bold' color='#2196F3'>{}</span>",
        info.manufacturer.trim()
    ));
    battery_box.append(&battery_main);

    // Espaceur pour pousser les infos secondaires vers le bas
    battery_box.append(&create_vertical_spacer());

    battery_box.append(&create_info_label(&format!("Nom: {}", info.name)));
    battery_box.append(&create_info_label(&format!("Modèle: {}", info.model_name)));
    battery_box.append(&create_info_label(&format!("Type: {}", info.technology)));
    row2.attach(&battery_frame, 2, 0, 1, 1);

    content_box.append(&row2);

    // === LIGNE 3: Électrique + Capacité + Infos ===
    let row3 = create_row_grid();

    // Card Électrique
    let (electrical_frame, electrical_box) = InfoCard::create("⚡ Électrique");
    electrical_box.append(&create_info_label(""));

    let power_main = Label::new(None);
    power_main.set_halign(gtk4::Align::Center);
    power_main.set_markup(&format!(
        "<span size='xx-large' weight='bold' color='orange'>{:.2}</span><span size='large'> W</span>",
        info.power_watts()
    ));
    electrical_box.append(&power_main);

    // Espaceur pour pousser les infos secondaires vers le bas
    electrical_box.append(&create_vertical_spacer());

    let voltage_value = create_info_label(&format!("Tension: {:.2} V", info.voltage_v()));
    electrical_box.append(&voltage_value);
    let current_value = create_info_label(&format!("Courant: {} mA", info.current_ma()));
    electrical_box.append(&current_value);
    let power_value = create_info_label(&format!("Puissance: {:.2} W", info.power_watts()));
    electrical_box.append(&power_value);
    row3.attach(&electrical_frame, 0, 0, 1, 1);

    // Card Capacité
    let (capacity_frame, capacity_box) = InfoCard::create("⚡ Capacité");
    capacity_box.append(&create_info_label(""));

    let capacity_main = Label::new(None);
    capacity_main.set_halign(gtk4::Align::Center);
    capacity_main.set_markup(&format!(
        "<span size='xx-large' weight='bold' color='#2196F3'>{}</span><span size='large'> mAh</span>",
        info.charge_now_mah()
    ));
    capacity_box.append(&capacity_main);

    // Espaceur pour pousser les infos secondaires vers le bas
    capacity_box.append(&create_vertical_spacer());

    let charge_now_value = create_info_label(&format!("Actuelle: {} mAh", info.charge_now_mah()));
    capacity_box.append(&charge_now_value);
    capacity_box.append(&create_info_label(&format!(
        "Max: {} mAh",
        info.charge_full_mah()
    )));
    capacity_box.append(&create_info_label(&format!(
        "Origine: {} mAh",
        info.charge_full_design_mah()
    )));
    row3.attach(&capacity_frame, 1, 0, 1, 1);

    // Card Service
    let (service_frame, service_box) = InfoCard::create("🔄 Service");
    service_box.append(&create_info_label(""));

    let service_label = Label::new(None);
    service_label.set_halign(gtk4::Align::Center);
    service_label.set_markup(&info.service_status_markup());
    service_box.append(&service_label);

    // Espaceur pour pousser les infos secondaires vers le bas
    service_box.append(&create_vertical_spacer());

    service_box.append(&create_info_label(""));
    service_box.append(&create_info_label(""));
    service_box.append(&create_info_label(""));

    row3.attach(&service_frame, 2, 0, 1, 1);

    content_box.append(&row3);

    // Créer la structure des widgets à mettre à jour
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
