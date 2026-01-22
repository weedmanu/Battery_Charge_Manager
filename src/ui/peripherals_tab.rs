//! Peripherals tab displaying wireless device batteries
//!
//! Shows battery status for detected peripheral devices (mouse, keyboard, etc.)
//! Each peripheral gets a single compact card with all information.

use gtk4::prelude::*;
use gtk4::{Box, Grid, Label};
use std::cell::Cell;

use crate::core::i18n::t;
use crate::core::PeripheralBattery;
use crate::ui::components::{create_content_box, InfoCard};

#[derive(Clone)]
pub struct UpdatablePeripheralsWidgets {
    pub devices: Vec<PeripheralDeviceWidgets>,
}

#[derive(Clone)]
pub struct PeripheralDeviceWidgets {
    pub stable_id: String,
    pub capacity_value: Label,
    pub status_value: Label,
    pub connection_value: Label,
    pub voltage_value: Label,
    pub name_value: Label,
    pub last_capacity: Cell<Option<u8>>,
}

fn remove_value_color_classes(label: &Label) {
    label.remove_css_class("color-success");
    label.remove_css_class("color-warning");
    label.remove_css_class("color-danger");
    label.remove_css_class("color-primary");
}

fn key_label(text: &str) -> Label {
    let label = Label::new(None);
    label.set_halign(gtk4::Align::Start);
    label.set_markup(&format!("<span weight='bold'>{text}</span>"));
    label
}

fn value_label(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.set_halign(gtk4::Align::Start);
    label
}

fn attach_kv_row(grid: &Grid, row: i32, key: &str, value: &Label) {
    let key = key_label(&format!("{key} :"));
    grid.attach(&key, 0, row, 1, 1);
    grid.attach(value, 1, row, 1, 1);
}

fn update_value_from_peripheral(widgets: &PeripheralDeviceWidgets, peripheral: &PeripheralBattery) {
    // Name (sysfs)
    widgets.name_value.set_text(&peripheral.name);

    let previous_capacity = widgets.last_capacity.get();
    widgets.last_capacity.set(Some(peripheral.capacity_percent));

    // Capacity
    widgets.capacity_value.set_markup(&format!(
        "<span size='large' weight='bold'>{} %</span>",
        peripheral.capacity_percent
    ));
    remove_value_color_classes(&widgets.capacity_value);
    widgets
        .capacity_value
        .add_css_class(peripheral.get_capacity_css_class());

    // Status
    // Some HID++ devices report `Unknown` when plugged via USB even while charging.
    // Use a small heuristic + capacity delta to show something useful.
    let raw_status = peripheral.status.trim();
    let (status_text, status_class) = if raw_status.eq_ignore_ascii_case("Full")
        || peripheral.capacity_percent >= 100
    {
        (t("full"), "color-success")
    } else if raw_status.eq_ignore_ascii_case("Charging") {
        (t("charging"), "color-primary")
    } else if raw_status.eq_ignore_ascii_case("Discharging") {
        (t("discharging"), "color-warning")
    } else if raw_status.eq_ignore_ascii_case("Unknown")
        && !peripheral.online
        && peripheral.capacity_percent < 100
    {
        // USB plugged case (best-effort)
        (format!("{} (?)", t("charging")), "color-primary")
    } else if raw_status.eq_ignore_ascii_case("Unknown") {
        match previous_capacity {
            Some(prev) if peripheral.capacity_percent > prev => (t("charging"), "color-primary"),
            Some(prev) if peripheral.capacity_percent < prev => (t("discharging"), "color-warning"),
            _ => (t("unknown"), "color-warning"),
        }
    } else {
        (raw_status.to_string(), peripheral.get_status_css_class())
    };

    widgets.status_value.set_text(&status_text);
    remove_value_color_classes(&widgets.status_value);
    widgets.status_value.add_css_class(status_class);

    // Connection
    let connection_text = if peripheral.is_connected() {
        t("connected")
    } else {
        t("disconnected")
    };
    widgets.connection_value.set_text(&connection_text);
    remove_value_color_classes(&widgets.connection_value);
    widgets
        .connection_value
        .add_css_class(if peripheral.is_connected() {
            "color-success"
        } else {
            "color-danger"
        });

    // Voltage
    widgets
        .voltage_value
        .set_text(&peripheral.get_voltage_string());
    remove_value_color_classes(&widgets.voltage_value);
    if peripheral.voltage_now.is_some() {
        widgets.voltage_value.add_css_class("color-primary");
    } else {
        widgets.voltage_value.add_css_class("color-warning");
    }
}

/// Builds the Peripherals tab content
///
/// # Arguments
///
/// * `peripherals` - List of detected peripheral batteries
///
/// # Returns
///
/// Tab Box containing all peripheral information + updatable widget handles
pub fn build_peripherals_tab(
    peripherals: &[PeripheralBattery],
) -> (Box, UpdatablePeripheralsWidgets) {
    crate::core::debug::debug_log_args(std::format_args!(
        "🕹️ [PERIPHERALS_TAB] Building peripherals tab with {} device(s)...",
        peripherals.len()
    ));
    let content_box = create_content_box(6);
    let mut updatable = UpdatablePeripheralsWidgets {
        devices: Vec::new(),
    };

    for peripheral in peripherals {
        // Une seule carte par périphérique
        let (device_frame, device_box) = InfoCard::create(&format!(
            "{} {}",
            peripheral.get_device_icon(),
            t("card_peripherals")
        ));

        // Marque et Modèle en grand en haut
        let manufacturer_label = Label::new(None);
        manufacturer_label.set_halign(gtk4::Align::Start);
        manufacturer_label.set_markup(&format!(
            "<span size='large' weight='bold'>{}</span>",
            peripheral.manufacturer
        ));
        manufacturer_label.add_css_class("color-primary");
        device_box.append(&manufacturer_label);

        let model_label = Label::new(None);
        model_label.set_halign(gtk4::Align::Start);
        model_label.set_markup(&format!(
            "<span size='medium'>{}</span>",
            peripheral.model_name
        ));
        model_label.set_margin_bottom(8);
        device_box.append(&model_label);

        // Grille 2 colonnes pour organiser les infos (Label : Valeur)
        let info_grid = Grid::new();
        info_grid.set_column_spacing(20);
        info_grid.set_row_spacing(4);
        info_grid.set_halign(gtk4::Align::Fill);

        // Make the value column expand a bit more nicely
        info_grid.set_column_homogeneous(false);

        let mut row = 0;

        // Capacity (dynamic)
        let capacity_value = Label::new(None);
        capacity_value.set_halign(gtk4::Align::Start);
        attach_kv_row(&info_grid, row, &t("capacity"), &capacity_value);
        row += 1;

        // Status (dynamic)
        let status_value = value_label(&peripheral.status);
        attach_kv_row(&info_grid, row, &t("status"), &status_value);
        row += 1;

        // Connection (dynamic)
        let connection_value = value_label("");
        attach_kv_row(&info_grid, row, &t("connection"), &connection_value);
        row += 1;

        // Voltage (dynamic)
        let voltage_value = value_label("");
        attach_kv_row(&info_grid, row, &t("voltage"), &voltage_value);
        row += 1;

        // Type (static)
        let type_value = value_label(&peripheral.device_type);
        attach_kv_row(&info_grid, row, &t("device_type"), &type_value);
        row += 1;

        // Scope (static)
        let scope_value = value_label(&peripheral.scope);
        attach_kv_row(&info_grid, row, &t("device_scope"), &scope_value);
        row += 1;

        // Name (dynamic - can change across modes)
        let name_value = value_label(&peripheral.name);
        attach_kv_row(&info_grid, row, &t("name"), &name_value);
        row += 1;

        // Serial (static if present)
        if let Some(serial) = peripheral.serial_number.as_deref() {
            let serial_value = value_label(serial);
            attach_kv_row(&info_grid, row, &t("serial_number"), &serial_value);
        }

        let device_widgets = PeripheralDeviceWidgets {
            stable_id: peripheral.stable_id(),
            capacity_value: capacity_value.clone(),
            status_value: status_value.clone(),
            connection_value: connection_value.clone(),
            voltage_value: voltage_value.clone(),
            name_value: name_value.clone(),
            last_capacity: Cell::new(None),
        };
        update_value_from_peripheral(&device_widgets, peripheral);
        updatable.devices.push(device_widgets);

        device_box.append(&info_grid);
        content_box.append(&device_frame);
    }

    (content_box, updatable)
}

pub fn update_peripherals_tab(widgets: &UpdatablePeripheralsWidgets) {
    let peripherals = PeripheralBattery::detect_all();

    crate::core::debug::debug_log_args(std::format_args!(
        "🔄 [UPDATE] Peripherals refresh: detected={} widgets={}",
        peripherals.len(),
        widgets.devices.len()
    ));

    for device_widgets in &widgets.devices {
        if let Some(peripheral) = peripherals
            .iter()
            .find(|p| p.stable_id() == device_widgets.stable_id)
        {
            update_value_from_peripheral(device_widgets, peripheral);
        } else {
            // Device disappeared; keep it visible but mark as disconnected.
            device_widgets.status_value.set_text("—");
            remove_value_color_classes(&device_widgets.status_value);
            device_widgets.status_value.add_css_class("color-warning");

            device_widgets.connection_value.set_text(&t("disconnected"));
            remove_value_color_classes(&device_widgets.connection_value);
            device_widgets
                .connection_value
                .add_css_class("color-danger");

            device_widgets.voltage_value.set_text("N/A");
            remove_value_color_classes(&device_widgets.voltage_value);
            device_widgets.voltage_value.add_css_class("color-warning");
        }
    }
}
