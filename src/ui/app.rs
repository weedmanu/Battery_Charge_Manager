//! Main application window and UI initialization
//!
//! Builds the GTK4 application window with notebook tabs for battery
//! information and settings. Manages auto-refresh timer.

use glib::timeout_add_local;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Label, Notebook, Orientation, Separator};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::core::i18n::t;
use crate::core::{BatteryInfo, PowerSupplyInfo};
use crate::ui::info_tab::build_info_tab;
use crate::ui::settings_tab::build_settings_tab;
use crate::ui::ui_tab::build_ui_tab;
use crate::{debug, debug_ui};

/// Builds the main application UI window
///
/// Creates a notebook with Information and Settings tabs. Shows
/// a fallback window if no battery is detected.
///
/// # Arguments
///
/// * `app` - GTK Application instance
pub fn build_ui(app: &Application) {
    let batteries = BatteryInfo::get_battery_list();

    if batteries.is_empty() {
        build_no_battery_window(app);
        return;
    }

    let current_battery = batteries[0].clone();
    debug!("Building UI for battery: {}", current_battery);

    let battery_info = match BatteryInfo::new(&current_battery) {
        Ok(info) => Rc::new(RefCell::new(info)),
        Err(e) => {
            eprintln!("{}: {}", t("error_battery_init"), e);
            build_no_battery_window(app);
            return;
        }
    };

    let window = ApplicationWindow::builder()
        .application(app)
        .title(t("app_title"))
        .default_width(800)
        .default_height(400)
        .resizable(false)
        .build();

    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(15);
    main_box.set_margin_bottom(15);
    main_box.set_margin_start(15);
    main_box.set_margin_end(15);

    // Header
    let header_label = Label::new(None);
    header_label.set_markup(&format!(
        "<span size='x-large' weight='bold'>🔋 {}</span>",
        t("app_title")
    ));
    main_box.append(&header_label);
    main_box.append(&Separator::new(Orientation::Horizontal));

    // Notebook (onglets)
    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    // Onglet Informations
    debug_ui!("Building information tab");
    let info = battery_info.borrow();
    let power_supply = PowerSupplyInfo::new();
    let (info_content, updatable_widgets) = build_info_tab(&info, &power_supply);
    drop(info);

    let info_tab_label = Label::new(Some(&format!("📊 {}", t("tab_info"))));
    notebook.append_page(&info_content, Some(&info_tab_label));

    // Onglet Réglages
    debug_ui!("Building settings tab");
    let settings_content = build_settings_tab(&battery_info.borrow(), &current_battery);
    let settings_tab_label = Label::new(Some(&format!("⚙️ {}", t("tab_settings"))));
    notebook.append_page(&settings_content, Some(&settings_tab_label));

    // Onglet Interface
    debug_ui!("Building UI preferences tab");
    let ui_content = build_ui_tab();
    let ui_tab_label = Label::new(Some(&format!("🎨 {}", t("tab_ui"))));
    notebook.append_page(&ui_content, Some(&ui_tab_label));

    main_box.append(&notebook);
    window.set_child(Some(&main_box));

    // Apply saved theme
    crate::ui::theme::apply_current_theme();

    // Auto-update toutes les 5 secondes
    setup_auto_update(battery_info.clone(), current_battery, updatable_widgets);

    window.present();
}

/// Displays fallback window when no battery is detected
///
/// # Arguments
///
/// * `app` - GTK Application instance
fn build_no_battery_window(app: &Application) {
    debug!("No battery detected, showing fallback window");

    let window = ApplicationWindow::builder()
        .application(app)
        .title(t("app_title"))
        .default_width(400)
        .default_height(200)
        .build();

    let label = Label::new(Some(&format!("⚠️ {}", t("no_battery"))));
    label.set_margin_top(20);
    label.set_margin_bottom(20);
    label.set_margin_start(20);
    label.set_margin_end(20);

    window.set_child(Some(&label));
    window.present();
}

/// Sets up automatic widget refresh timer
///
/// Refreshes battery information every 5 seconds.
///
/// # Arguments
///
/// * `battery_info` - Shared battery information
/// * `current_battery` - Battery name to monitor
/// * `widgets` - Updatable widget references
fn setup_auto_update(
    battery_info: Rc<RefCell<BatteryInfo>>,
    current_battery: String,
    widgets: crate::ui::components::UpdatableWidgets,
) {
    debug_ui!("Setting up 5-second auto-refresh timer");

    timeout_add_local(
        Duration::from_secs(5),
        glib::clone!(
            #[weak(rename_to = capacity_label)]
            widgets.capacity_label,
            #[weak(rename_to = health_label)]
            widgets.health_label,
            #[weak(rename_to = status_value)]
            widgets.status_value,
            #[weak(rename_to = voltage_value)]
            widgets.voltage_value,
            #[weak(rename_to = current_value)]
            widgets.current_value,
            #[weak(rename_to = power_value)]
            widgets.power_value,
            #[weak(rename_to = charge_now_value)]
            widgets.charge_now_value,
            #[weak(rename_to = power_source_value)]
            widgets.power_source_value,
            #[weak(rename_to = threshold_stop_label)]
            widgets.threshold_stop_label,
            #[weak(rename_to = service_label)]
            widgets.service_label,
            #[upgrade_or]
            glib::ControlFlow::Break,
            move || {
                let threshold_start_opt = widgets.threshold_start_label.clone();
                let alarm_opt = widgets.alarm_label.clone();

                let info = match BatteryInfo::new(&current_battery) {
                    Ok(info) => info,
                    Err(e) => {
                        eprintln!("Erreur lors du rafraîchissement: {}", e);
                        return glib::ControlFlow::Continue;
                    }
                };
                let power_supply = PowerSupplyInfo::new();

                // Mise à jour alimentation
                power_source_value.set_markup(power_supply.get_power_source_markup());

                // Mise à jour état
                status_value.set_markup(&info.get_status_markup());

                // Mise à jour labels
                capacity_label.set_markup(&format!(
                    "<span size='xx-large' weight='bold' color='#2196F3'>{}</span><span size='large'>%</span>",
                    info.capacity_percent
                ));

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

                // Mise à jour valeurs électriques
                voltage_value.set_text(&format!("{}: {:.2} V", t("voltage"), info.voltage_v()));
                current_value.set_text(&format!("{}: {} mA", t("current"), info.current_ma()));
                power_value.set_text(&format!("{}: {:.2} W", t("power"), info.power_watts()));
                charge_now_value.set_text(&format!(
                    "{}: {} mAh",
                    t("current_capacity"),
                    info.charge_now_mah()
                ));

                // Mise à jour des seuils
                if let Some(ref start_label) = threshold_start_opt {
                    if let Some(start_val) = info.charge_start_threshold {
                        start_label.set_markup(&format!(
                            "<span size='x-large' weight='bold' color='#2196F3'>{}%</span>",
                            start_val
                        ));
                    }
                }

                threshold_stop_label.set_markup(&format!(
                    "<span size='x-large' weight='bold' color='#4CAF50'>{}</span>",
                    info.charge_stop_threshold
                        .map(|v| format!("{}%", v))
                        .unwrap_or_else(|| "N/A".to_string())
                ));

                // Mise à jour alarme
                if let Some(ref alarm_label) = alarm_opt {
                    if let Some(alarm_pct) = info.alarm_percent() {
                        alarm_label.set_markup(&format!(
                            "<span size='x-large' weight='bold' color='#FF5722'>{:.1}%</span>",
                            alarm_pct
                        ));
                    }
                }

                // Mise à jour état du service
                service_label.set_markup(&info.service_status_markup());

                *battery_info.borrow_mut() = info;

                glib::ControlFlow::Continue
            }
        ),
    );
}
