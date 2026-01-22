//! Battery Manager - Application GTK4 de gestion de batterie
//!
//! Structure SOLID:
//! - core/ : Logique métier (`BatteryInfo`, `PowerSupplyInfo`, `battery_control`)
//! - ui/ : Interface utilisateur (GTK4)

mod core;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "com.battery.manager";

fn main() {
    // L'application démarre sans privilèges root
    // pkexec sera demandé uniquement lors du clic sur "Appliquer les seuils"

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(ui::build_ui);
    app.run();
}
