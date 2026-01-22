//! Peripheral battery detection module
//!
//! Detects and monitors wireless peripheral devices (mouse, keyboard, etc.)
//! with battery capability via HID++ protocol or similar interfaces.

use std::collections::BTreeMap;
use std::fs;

/// Peripheral device battery information (read-only)
#[derive(Debug, Clone)]
pub struct PeripheralBattery {
    pub name: String,
    pub manufacturer: String,
    pub model_name: String,
    pub status: String,
    pub capacity_percent: u8,
    pub voltage_now: Option<u64>,
    pub serial_number: Option<String>,
    pub online: bool,
    pub device_type: String,
    pub scope: String,
}

impl PeripheralBattery {
    fn dedupe_score(&self) -> u32 {
        let mut value = 0;

        if self.online {
            value += 16;
        }

        if self.voltage_now.is_some() {
            value += 8;
        }

        if self.capacity_percent > 0 {
            value += 4;
        }

        if !self.status.trim().eq_ignore_ascii_case("Unknown") {
            value += 2;
        }

        if self.status.trim().eq_ignore_ascii_case("Charging") {
            value += 2;
        }

        value
    }

    /// Creates a new instance by reading sysfs files for a peripheral device
    ///
    /// # Arguments
    ///
    /// * `device_name` - Device name from `/sys/class/power_supply/` (e.g., "`hidpp_battery_1`")
    ///
    /// # Returns
    ///
    /// `PeripheralBattery` with read-only information (no threshold modification)
    pub fn new(device_name: &str) -> Self {
        let base_path = format!("/sys/class/power_supply/{device_name}");

        let manufacturer = Self::read_sysfs_string(&base_path, "manufacturer")
            .unwrap_or_else(|_| "Unknown".to_string());

        let model_name = Self::read_sysfs_string(&base_path, "model_name")
            .unwrap_or_else(|_| "Unknown".to_string());

        let status =
            Self::read_sysfs_string(&base_path, "status").unwrap_or_else(|_| "Unknown".to_string());

        let capacity_percent = Self::read_sysfs_u8(&base_path, "capacity").unwrap_or(0);

        let voltage_now = Self::read_sysfs_u64(&base_path, "voltage_now").ok();

        let serial_number = Self::read_sysfs_string(&base_path, "serial_number").ok();

        let online = Self::read_sysfs_u8(&base_path, "online").unwrap_or(0) == 1;

        let device_type =
            Self::read_sysfs_string(&base_path, "type").unwrap_or_else(|_| "Unknown".to_string());

        let scope =
            Self::read_sysfs_string(&base_path, "scope").unwrap_or_else(|_| "Unknown".to_string());

        Self {
            name: device_name.to_string(),
            manufacturer,
            model_name,
            status,
            capacity_percent,
            voltage_now,
            serial_number,
            online,
            device_type,
            scope,
        }
    }

    /// Scans `/sys/class/power_supply/` for peripheral batteries
    ///
    /// Detects devices matching patterns: `hidpp_battery_*`, `hid-*-battery`, etc.
    ///
    /// # Returns
    ///
    /// Vector of `PeripheralBattery` instances for all detected peripheral devices
    pub fn detect_all() -> Vec<Self> {
        let mut best_by_id: BTreeMap<String, Self> = BTreeMap::new();
        let mut matched_entries = 0usize;

        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();

                // Filtrer les périphériques (souris, clavier via HID++)
                if name.starts_with("hidpp_battery_")
                    || name.starts_with("hid-")
                    || name.contains("mouse")
                    || name.contains("keyboard")
                {
                    matched_entries += 1;
                    let device = Self::new(&name);
                    let id = device.stable_id();

                    match best_by_id.get(&id) {
                        None => {
                            best_by_id.insert(id, device);
                        }
                        Some(existing) => {
                            let existing_score = existing.dedupe_score();
                            let new_score = device.dedupe_score();

                            if new_score > existing_score
                                || (new_score == existing_score && device.name < existing.name)
                            {
                                best_by_id.insert(id, device);
                            }
                        }
                    }
                }
            }
        }

        if crate::core::debug::is_debug_enabled() {
            crate::core::debug::debug_log_args(std::format_args!(
                "🖱️ [PERIPHERALS] matched_entries={matched_entries} unique_devices={} (after dedupe)",
                best_by_id.len()
            ));
        }

        best_by_id.into_values().collect()
    }

    /// Reads a sysfs file and returns the content as a trimmed String
    fn read_sysfs_string(base_path: &str, filename: &str) -> Result<String, std::io::Error> {
        let path = format!("{base_path}/{filename}");
        let content = fs::read_to_string(&path)?;
        Ok(content.trim().to_string())
    }

    /// Reads a sysfs file and parses it as u8
    fn read_sysfs_u8(base_path: &str, filename: &str) -> Result<u8, std::io::Error> {
        let content = Self::read_sysfs_string(base_path, filename)?;
        content
            .parse::<u8>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Reads a sysfs file and parses it as u64
    fn read_sysfs_u64(base_path: &str, filename: &str) -> Result<u64, std::io::Error> {
        let content = Self::read_sysfs_string(base_path, filename)?;
        content
            .parse::<u64>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Returns CSS class for capacity color (≥80% success, 20-79% warning, <20% danger)
    pub const fn get_capacity_css_class(&self) -> &str {
        if self.capacity_percent >= 80 {
            "color-success"
        } else if self.capacity_percent >= 20 {
            "color-warning"
        } else {
            "color-danger"
        }
    }

    /// Returns a stable identifier for matching the same device across refreshes.
    ///
    /// Some devices can change their `/sys/class/power_supply/*` name depending on mode
    /// (wired vs wireless, receiver reconnect, etc). Prefer serial number when available.
    pub fn stable_id(&self) -> String {
        let serial = self.serial_number.as_deref().unwrap_or("").trim();
        if !serial.is_empty() && serial != "Unknown" {
            return format!("serial:{serial}");
        }

        let manufacturer = self.manufacturer.trim();
        let model_name = self.model_name.trim();
        if manufacturer != "Unknown" && model_name != "Unknown" {
            return format!("mm:{manufacturer}|{model_name}");
        }

        format!("name:{}", self.name)
    }

    /// Best-effort connection state.
    ///
    /// `online` is not always reliable across device modes; when charging via cable,
    /// some devices may flip `online` while still being present and reporting a status.
    pub fn is_connected(&self) -> bool {
        if self.online {
            return true;
        }

        if self.voltage_now.is_some() {
            return true;
        }

        if self.capacity_percent > 0 {
            return true;
        }

        let status = self.status.trim();
        !status.is_empty() && !status.eq_ignore_ascii_case("Unknown")
    }

    /// Returns formatted voltage string
    ///
    /// # Returns
    ///
    /// Voltage in volts (e.g., "4.23 V") or "N/A"
    #[allow(clippy::cast_precision_loss)]
    pub fn get_voltage_string(&self) -> String {
        self.voltage_now.map_or_else(
            || "N/A".to_string(),
            |v| format!("{:.2} V", v as f64 / 1_000_000.0),
        )
    }

    /// Returns device icon emoji based on name/type
    ///
    /// # Returns
    ///
    /// "🖱️" for mouse, "⌨️" for keyboard, "🔋" for generic
    pub fn get_device_icon(&self) -> &'static str {
        let name_lower = self.name.to_lowercase();
        if name_lower.contains("mouse") || self.model_name.to_lowercase().contains("mouse") {
            "🖱️"
        } else if name_lower.contains("keyboard")
            || self.model_name.to_lowercase().contains("keyboard")
        {
            "⌨️"
        } else {
            "🔋"
        }
    }

    /// Returns CSS class for status color
    ///
    /// - Charging: primary
    /// - Full: success
    /// - Discharging / Not charging: warning
    /// - Unknown/other: warning
    pub const fn get_status_css_class(&self) -> &'static str {
        let status = self.status.as_str();
        if status.eq_ignore_ascii_case("Charging") {
            "color-primary"
        } else if status.eq_ignore_ascii_case("Full") {
            "color-success"
        } else {
            "color-warning"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peripheral_detection() {
        // Test de détection (peut être vide si aucun périphérique)
        let peripherals = PeripheralBattery::detect_all();
        // Simplement vérifier que la fonction ne panic pas
        let _count = peripherals.len();
    }

    #[test]
    fn test_capacity_css_class() {
        let peripheral = PeripheralBattery {
            name: "test".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test Device".to_string(),
            status: "Discharging".to_string(),
            capacity_percent: 85,
            voltage_now: None,
            serial_number: Some("ABC-123".to_string()),
            online: true,
            device_type: "Battery".to_string(),
            scope: "Device".to_string(),
        };

        // Vérifier que la classe CSS est correcte (85% = success)
        assert_eq!(peripheral.get_capacity_css_class(), "color-success");
    }

    #[test]
    fn test_device_icon() {
        let mouse = PeripheralBattery {
            name: "hidpp_battery_1".to_string(),
            manufacturer: "Logitech".to_string(),
            model_name: "G Pro Wireless Gaming Mouse".to_string(),
            status: "Discharging".to_string(),
            capacity_percent: 100,
            voltage_now: Some(4_229_000),
            serial_number: Some("e9-97-e3-8b".to_string()),
            online: true,
            device_type: "Battery".to_string(),
            scope: "Device".to_string(),
        };

        assert_eq!(mouse.get_device_icon(), "🖱️");
    }
}
