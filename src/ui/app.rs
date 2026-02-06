//! Main application window and UI initialization
//!
//! Builds the GTK4 application window with notebook tabs for battery
//! information and settings. Manages auto-refresh timer.

use glib::timeout_add_local;
use gtk4::prelude::*;
use gtk4::{
    gio, AboutDialog, Application, ApplicationWindow, Box, HeaderBar, Label, MenuButton, Notebook,
    Orientation, Separator,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use crate::core::i18n::t;
use crate::core::{BatteryInfo, PeripheralBattery, PowerSupplyInfo};
use crate::debug_ui;
use crate::ui::info_tab::build_info_tab;
use crate::ui::peripherals_tab::{
    build_peripherals_tab, update_peripherals_tab, UpdatablePeripheralsWidgets,
};
use crate::ui::settings_tab::build_settings_tab;
use crate::ui::ui_tab::build_ui_tab;

fn find_installed_doc(filename: &str) -> Option<PathBuf> {
    let candidates = [
        format!("/usr/share/battery-manager/docs/{filename}"),
        format!("/usr/share/doc/battery-manager/{filename}"),
        format!("docs/{filename}"),
    ];

    candidates
        .into_iter()
        .map(PathBuf::from)
        .find(|p| p.exists())
}

fn open_doc(filename: &str) {
    let Some(path) = find_installed_doc(filename) else {
        crate::core::debug::terminal_error_args(std::format_args!(
            "❌ [DOCS] {}",
            t("docs_not_found")
        ));
        return;
    };

    let uri = format!("file://{}", path.display());
    if gio::AppInfo::launch_default_for_uri(&uri, None::<&gio::AppLaunchContext>).is_err() {
        crate::core::debug::terminal_error_args(std::format_args!(
            "❌ [DOCS] {}",
            t("docs_open_failed")
        ));
    }
}

fn ensure_help_menu(app: &Application, window: &ApplicationWindow) {
    if app.lookup_action("about").is_some() {
        return;
    }

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(glib::clone!(
        #[weak]
        window,
        move |_, _| {
            let about = AboutDialog::builder()
                .transient_for(&window)
                .modal(true)
                .program_name("Battery Manager")
                .version(env!("CARGO_PKG_VERSION"))
                .comments(t("about_text"))
                .license_type(gtk4::License::MitX11)
                .website("https://github.com/weedmanu/Battery_Charge_Manager")
                .website_label("GitHub")
                .build();
            about.present();
        }
    ));
    app.add_action(&about_action);

    let readme_action = gio::SimpleAction::new("open_readme", None);
    readme_action.connect_activate(move |_, _| {
        open_doc("README.html");
    });
    app.add_action(&readme_action);

    let references_action = gio::SimpleAction::new("open_references", None);
    references_action.connect_activate(move |_, _| {
        open_doc("REFERENCES.html");
    });
    app.add_action(&references_action);
}

/// Builds the main application UI window
///
/// Creates a notebook with Information and Settings tabs. Shows
/// a fallback window if no battery is detected.
///
/// # Arguments
///
/// * `app` - GTK Application instance
#[allow(clippy::too_many_lines)]
pub fn build_ui(app: &Application) {
    crate::core::debug::debug_log("🚀 [APP] Starting UI build...");
    let batteries = BatteryInfo::get_battery_list();
    crate::core::debug::debug_log_args(std::format_args!(
        "🔋 [APP] Detected {} battery/batteries",
        batteries.len()
    ));

    if batteries.is_empty() {
        crate::core::debug::debug_log("⚠️ [APP] No battery detected, showing fallback window");
        build_no_battery_window(app);
        return;
    }

    let current_battery = batteries[0].clone();
    crate::core::debug::debug_log_args(std::format_args!(
        "🔋 [APP] Building UI for battery: {current_battery}"
    ));

    let battery_info = match BatteryInfo::new(&current_battery) {
        Ok(info) => Rc::new(RefCell::new(info)),
        Err(e) => {
            crate::core::debug::terminal_error_args(std::format_args!(
                "❌ [APP] {}: {e}",
                t("error_battery_init")
            ));
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

    ensure_help_menu(app, &window);

    // Header bar with Help menu
    let header_bar = HeaderBar::new();
    let menu = gio::Menu::new();
    let help_menu = gio::Menu::new();
    help_menu.append(Some(t("open_readme").as_str()), Some("app.open_readme"));
    help_menu.append(
        Some(t("open_references").as_str()),
        Some("app.open_references"),
    );
    help_menu.append(Some(t("about").as_str()), Some("app.about"));
    menu.append_section(Some(t("help").as_str()), &help_menu);

    let help_button = MenuButton::builder()
        .icon_name("help-about-symbolic")
        .build();
    help_button.set_menu_model(Some(&menu));
    help_button.set_tooltip_text(Some(t("help").as_str()));
    header_bar.pack_end(&help_button);
    window.set_titlebar(Some(&header_bar));

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

    // Onglet Périphériques (si détectés)
    debug_ui!("Checking for peripheral devices");
    let peripherals = PeripheralBattery::detect_all();
    let mut peripherals_widgets: Option<UpdatablePeripheralsWidgets> = None;
    if !peripherals.is_empty() {
        debug_ui!("Building peripherals tab ({} device(s))", peripherals.len());
        let (peripherals_content, widgets) = build_peripherals_tab(&peripherals);
        peripherals_widgets = Some(widgets);
        let peripherals_tab_label = Label::new(Some(&format!("🖱️ {}", t("tab_peripherals"))));
        notebook.append_page(&peripherals_content, Some(&peripherals_tab_label));
    }

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

    // Debug: log tab switches (useful with `--debug`)
    notebook.connect_switch_page(|nb, page, page_num| {
        let tab_label = nb
            .tab_label(page)
            .and_then(|w| w.downcast::<Label>().ok())
            .map_or_else(|| format!("page-{page_num}"), |l| l.text().to_string());
        debug_ui!("Switched tab -> #{page_num} ({tab_label})");
    });

    main_box.append(&notebook);
    window.set_child(Some(&main_box));

    // Apply saved theme
    crate::ui::theme::apply_current_theme();

    // Auto-update toutes les 5 secondes
    setup_auto_update(
        battery_info.clone(),
        current_battery,
        updatable_widgets,
        peripherals_widgets,
    );

    window.present();
}

/// Displays fallback window when no battery is detected
///
/// # Arguments
///
/// * `app` - GTK Application instance
fn build_no_battery_window(app: &Application) {
    crate::core::debug::debug_log("⚠️ [APP] No battery detected, showing fallback window");

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
#[allow(clippy::too_many_lines)]
fn setup_auto_update(
    battery_info: Rc<RefCell<BatteryInfo>>,
    current_battery: String,
    widgets: crate::ui::components::UpdatableWidgets,
    peripherals_widgets: Option<UpdatablePeripheralsWidgets>,
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
                        crate::core::debug::terminal_error_args(std::format_args!(
                            "❌ [UPDATE] Error during refresh: {e}"
                        ));
                        return glib::ControlFlow::Continue;
                    }
                };
                let power_supply = PowerSupplyInfo::new();

                if let Some(ref peripherals_widgets) = peripherals_widgets {
                    update_peripherals_tab(peripherals_widgets);
                }

                // Update power supply
                power_source_value.set_markup(&power_supply.get_power_source_markup());
                power_source_value.remove_css_class("color-success");
                power_source_value.remove_css_class("color-warning");
                power_source_value.add_css_class(power_supply.get_power_source_css_class());

                // Update status
                status_value.set_markup(&info.get_status_markup());
                // Remove old classes and add new one
                status_value.remove_css_class("color-success");
                status_value.remove_css_class("color-warning");
                status_value.remove_css_class("color-primary");
                let status_class = info.get_status_css_class();
                status_value.add_css_class(status_class);
                crate::core::debug::debug_log_args(std::format_args!(
                    "🔄 [UPDATE] Status class updated to: {status_class}"
                ));

                // Update labels
                capacity_label.set_markup(&format!(
                    "<span size='xx-large' weight='bold'>{}</span><span size='large'>%</span>",
                    info.capacity_percent
                ));
                // Note: capacity_label keeps color-primary class, no update needed

                health_label.set_markup(&format!(
                    "<span size='xx-large' weight='bold'>{:.1}</span><span size='large'>%</span>",
                    info.health_percent
                ));
                // Remove old classes and add new one
                health_label.remove_css_class("color-success");
                health_label.remove_css_class("color-warning");
                health_label.remove_css_class("color-danger");
                let health_class = info.get_health_css_class();
                health_label.add_css_class(health_class);
                crate::core::debug::debug_log_args(std::format_args!(
                    "🔄 [UPDATE] Health class updated to: {health_class}"
                ));

                // Update electrical values
                voltage_value.set_text(&format!("{}: {:.2} V", t("voltage"), info.voltage_v()));
                current_value.set_text(&format!("{}: {} mA", t("current"), info.current_ma()));
                power_value.set_text(&format!("{}: {:.2} W", t("power"), info.power_watts()));
                charge_now_value.set_text(&format!(
                    "{}: {} mAh",
                    t("current_capacity"),
                    info.charge_now_mah()
                ));

                // Update thresholds
                if let Some(ref start_label) = threshold_start_opt {
                    if let Some(start_val) = info.charge_start_threshold {
                        start_label.set_markup(&format!(
                            "<span size='x-large' weight='bold'>{start_val}%</span>"
                        ));
                        // Note: start_label keeps color-primary class
                    }
                }

                threshold_stop_label.set_markup(&format!(
                    "<span size='x-large' weight='bold'>{}</span>",
                    info.charge_stop_threshold
                        .map_or_else(|| "N/A".to_string(), |v| format!("{v}%"))
                ));
                // Note: threshold_stop_label garde sa classe color-success

                // Update alarm
                if let Some(ref alarm_label) = alarm_opt {
                    if let Some(alarm_pct) = info.alarm_percent() {
                        alarm_label.set_markup(&format!(
                            "<span size='x-large' weight='bold'>{alarm_pct:.1}%</span>"
                        ));
                        // Note: alarm_label keeps color-danger class
                    }
                }

                // Update service status
                service_label.set_markup(&info.service_status_markup());
                // Remove old classes and add new one
                service_label.remove_css_class("color-success");
                service_label.remove_css_class("color-danger");
                let service_class = info.service_status_css_class();
                service_label.add_css_class(service_class);
                crate::core::debug::debug_log_args(std::format_args!(
                    "🔄 [UPDATE] Service class updated to: {service_class}"
                ));

                *battery_info.borrow_mut() = info;

                glib::ControlFlow::Continue
            }
        ),
    );
}
