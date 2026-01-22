//! Power supply detection module
//!
//! Provides AC power (mains) detection and status information.

use std::fs;

/// AC power supply information
#[derive(Debug, Clone)]
pub struct PowerSupplyInfo {
    pub ac_online: bool,
    pub ac_name: String,
}

impl PowerSupplyInfo {
    /// Creates a new instance by detecting AC power status
    ///
    /// Scans `/sys/class/power_supply/` for "Mains" type devices
    ///
    /// # Returns
    ///
    /// `PowerSupplyInfo` with AC status and device name
    pub fn new() -> Self {
        let mut ac_online = false;
        let mut ac_name = String::from("Non détecté");
        let mut found_mains = false;

        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let type_path = format!("/sys/class/power_supply/{name}/type");
                if let Ok(psu_type) = fs::read_to_string(&type_path) {
                    if psu_type.trim() == "Mains" {
                        ac_name.clone_from(&name);
                        found_mains = true;
                        let online_path = format!("/sys/class/power_supply/{name}/online");
                        if let Ok(online) = fs::read_to_string(&online_path) {
                            ac_online = online.trim() == "1";
                        }
                        break;
                    }
                }
            }
        }

        if crate::core::debug::is_debug_enabled() {
            if found_mains {
                crate::core::debug::debug_log_args(std::format_args!(
                    "🔌 [POWER] Mains={ac_name} online={ac_online}"
                ));
            } else {
                crate::core::debug::debug_log("🔌 [POWER] No 'Mains' power supply found");
            }
        }

        Self { ac_online, ac_name }
    }

    /// Returns markup string for power source display
    ///
    /// # Returns
    ///
    /// Pango markup with green "🔌 Secteur" or orange "🔋 Batterie"
    pub const fn get_power_source_markup(&self) -> &'static str {
        if self.ac_online {
            "<span size='xx-large' weight='bold' color='green'>🔌 Secteur</span>"
        } else {
            "<span size='xx-large' weight='bold' color='orange'>🔋 Batterie</span>"
        }
    }
}

impl Default for PowerSupplyInfo {
    fn default() -> Self {
        Self::new()
    }
}
