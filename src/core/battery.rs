//! Battery information module
//!
//! Provides battery status, capacity, health, and threshold information
//! by reading from `/sys/class/power_supply/` sysfs interface.

use std::fs;

use crate::core::i18n::t;

// Note: Markup functions are no longer used directly.
// Colors are now dynamically managed via crate::ui::theme

/// Errors that can occur when creating a `BatteryInfo` instance
#[derive(Debug)]
pub enum BatteryError {
    /// Invalid battery name (must start with "BAT")
    InvalidBatteryName(String),
    /// I/O error when reading sysfs files
    IoError(std::io::Error),
}

impl std::fmt::Display for BatteryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBatteryName(name) => {
                write!(
                    f,
                    "Nom de batterie invalide: '{name}'. Le nom doit commencer par 'BAT'."
                )
            }
            Self::IoError(e) => write!(f, "I/O Error: {e}"),
        }
    }
}

impl std::error::Error for BatteryError {}

impl From<std::io::Error> for BatteryError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

/// Detailed battery information
///
/// Contains all battery metrics including status, capacity, health,
/// electrical parameters, and charge thresholds.
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub name: String,
    pub manufacturer: String,
    pub model_name: String,
    pub technology: String,
    pub status: String,
    pub capacity_percent: u8,
    pub capacity_level: String,
    pub charge_now: u64,
    pub charge_full: u64,
    pub charge_full_design: u64,
    pub current_now: u64,
    pub voltage_now: u64,
    pub cycle_count: u32,
    pub health_percent: f32,
    pub wear_percent: f32,
    pub time_remaining_minutes: Option<u32>,
    pub charge_start_threshold: Option<u8>,
    pub charge_stop_threshold: Option<u8>,
    pub alarm: Option<u64>,
    pub service_active: bool,
}

impl BatteryInfo {
    /// Creates a new `BatteryInfo` instance by reading sysfs files
    ///
    /// # Arguments
    ///
    /// * `battery_name` - Battery name (must start with "BAT", e.g., "BAT0", "BAT1")
    ///
    /// # Returns
    ///
    /// * `Ok(BatteryInfo)` - Successfully read battery information
    /// * `Err(BatteryError)` - Invalid battery name or I/O error
    ///
    /// # Errors
    ///
    /// Returns `BatteryError::InvalidBatteryName` if:
    /// - Name doesn't start with "BAT"
    /// - Name contains path traversal sequences ("../", "./")
    /// - Name contains directory separators
    ///
    /// # Security
    ///
    /// This function validates the battery name to prevent path traversal attacks
    #[allow(clippy::too_many_lines)]
    pub fn new(battery_name: &str) -> Result<Self, BatteryError> {
        // Validate battery name to prevent path traversal
        if !battery_name.starts_with("BAT") {
            return Err(BatteryError::InvalidBatteryName(battery_name.to_string()));
        }

        // Security: Prevent path traversal attacks
        if battery_name.contains("..") || battery_name.contains('/') || battery_name.contains('\\')
        {
            return Err(BatteryError::InvalidBatteryName(format!(
                "Invalid battery name (potential path traversal): {battery_name}"
            )));
        }

        let base_path = format!("/sys/class/power_supply/{battery_name}");

        if crate::core::debug::is_debug_enabled() {
            crate::core::debug::debug_log_args(std::format_args!(
                "🔍 [BATTERY] Reading sysfs for {battery_name}"
            ));
        }

        let name = battery_name.to_string();

        let manufacturer = Self::read_sys_file(&format!("{base_path}/manufacturer"))
            .unwrap_or_else(|| "Inconnu".to_string());
        let model_name = Self::read_sys_file(&format!("{base_path}/model_name"))
            .unwrap_or_else(|| "Inconnu".to_string());
        let technology = Self::read_sys_file(&format!("{base_path}/technology"))
            .unwrap_or_else(|| "Inconnu".to_string());
        let status = Self::read_sys_file(&format!("{base_path}/status"))
            .unwrap_or_else(|| "Inconnu".to_string());
        let capacity_level = Self::read_sys_file(&format!("{base_path}/capacity_level"))
            .unwrap_or_else(|| "Inconnu".to_string());

        let capacity_percent = Self::read_sys_file(&format!("{base_path}/capacity"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        if crate::core::debug::is_debug_enabled() {
            crate::core::debug::debug_log_args(std::format_args!(
                "🔋 [BATTERY] {battery_name}: status={status}, capacity={capacity_percent}%"
            ));
        }

        let charge_now = Self::read_sys_file(&format!("{base_path}/charge_now"))
            .or_else(|| Self::read_sys_file(&format!("{base_path}/energy_now")))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let charge_full = Self::read_sys_file(&format!("{base_path}/charge_full"))
            .or_else(|| Self::read_sys_file(&format!("{base_path}/energy_full")))
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        let charge_full_design = Self::read_sys_file(&format!("{base_path}/charge_full_design"))
            .or_else(|| Self::read_sys_file(&format!("{base_path}/energy_full_design")))
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        let current_now = Self::read_sys_file(&format!("{base_path}/current_now"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let voltage_now = Self::read_sys_file(&format!("{base_path}/voltage_now"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let cycle_count = Self::read_sys_file(&format!("{base_path}/cycle_count"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let health_percent = if charge_full_design > 0 {
            #[allow(clippy::cast_precision_loss)]
            let result = (charge_full as f32 / charge_full_design as f32) * 100.0;
            result
        } else {
            100.0
        };

        let wear_percent = 100.0 - health_percent;

        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let time_remaining_minutes = if current_now > 0 && status == "Discharging" {
            Some((charge_now as f32 / current_now as f32 * 60.0) as u32)
        } else if current_now > 0 && status == "Charging" {
            Some(((charge_full - charge_now) as f32 / current_now as f32 * 60.0) as u32)
        } else {
            None
        };

        let charge_start_threshold =
            Self::read_sys_file(&format!("{base_path}/charge_start_threshold"))
                .or_else(|| {
                    Self::read_sys_file(&format!("{base_path}/charge_control_start_threshold"))
                })
                .and_then(|s| s.parse().ok());

        let charge_stop_threshold =
            Self::read_sys_file(&format!("{base_path}/charge_stop_threshold"))
                .or_else(|| {
                    Self::read_sys_file(&format!("{base_path}/charge_control_end_threshold"))
                })
                .and_then(|s| s.parse().ok());

        let alarm = Self::read_sys_file(&format!("{base_path}/alarm")).and_then(|s| s.parse().ok());

        // Vérifier si le service systemd battery-manager est actif
        let service_active = std::process::Command::new("systemctl")
            .args(["is-active", "battery-manager.service"])
            .output()
            .ok()
            .is_some_and(|output| output.status.success());

        if crate::core::debug::is_debug_enabled() {
            crate::core::debug::debug_log_args(std::format_args!(
                "🎯 [BATTERY] thresholds: start={charge_start_threshold:?} stop={charge_stop_threshold:?} alarm={alarm:?} service_active={service_active}"
            ));
        }

        Ok(Self {
            name,
            manufacturer,
            model_name,
            technology,
            status,
            capacity_percent,
            capacity_level,
            charge_now,
            charge_full,
            charge_full_design,
            current_now,
            voltage_now,
            cycle_count,
            health_percent,
            wear_percent,
            time_remaining_minutes,
            charge_start_threshold,
            charge_stop_threshold,
            alarm,
            service_active,
        })
    }

    /// Reads a sysfs file and returns its trimmed content
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the sysfs file
    ///
    /// # Returns
    ///
    /// * `Some(String)` - File content (trimmed)
    /// * `None` - File doesn't exist or read error
    fn read_sys_file(path: &str) -> Option<String> {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    }

    /// Returns the list of available batteries
    ///
    /// Scans `/sys/class/power_supply/` for devices starting with "BAT"
    ///
    /// # Returns
    ///
    /// Sorted vector of battery names (e.g., `["BAT0", "BAT1"]`)
    pub fn get_battery_list() -> Vec<String> {
        let mut batteries = Vec::new();
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("BAT") {
                    batteries.push(name);
                }
            }
        }
        batteries.sort();

        if crate::core::debug::is_debug_enabled() {
            crate::core::debug::debug_log_args(std::format_args!(
                "🔎 [BATTERY] Detected {} battery/batteries",
                batteries.len()
            ));
        }

        batteries
    }

    /// Returns formatted status text with markup for display
    ///
    /// # Returns
    ///
    /// Pango markup string with color and icon for battery status
    pub fn get_status_markup(&self) -> String {
        match self.status.as_str() {
            "Charging" => format!(
                "<span size='xx-large' weight='bold'>⚡ {}</span>",
                t("charging")
            ),
            "Discharging" => format!(
                "<span size='xx-large' weight='bold'>🔋 {}</span>",
                t("discharging")
            ),
            "Full" => format!("<span size='xx-large' weight='bold'>✓ {}</span>", t("full")),
            "Not charging" => {
                if self.capacity_percent >= 100 {
                    format!("<span size='xx-large' weight='bold'>✓ {}</span>", t("full"))
                } else {
                    format!(
                        "<span size='xx-large' weight='bold'>⏸️ {}</span>",
                        t("not_charging")
                    )
                }
            }
            _ => format!(
                "<span size='xx-large' weight='bold'>? {}</span>",
                t("unknown")
            ),
        }
    }

    /// Returns CSS class for battery status color
    ///
    /// # Returns
    ///
    /// CSS class name ("color-success", "color-warning", "color-primary", "color-danger")
    pub fn get_status_css_class(&self) -> &str {
        match self.status.as_str() {
            "Charging" => "color-success",
            "Full" | "Not charging" => "color-primary",
            _ => "color-warning",
        }
    }

    /// Returns CSS class for health percentage color
    ///
    /// # Returns
    ///
    /// CSS class name ("color-success" ≥80%, "color-warning" 60-79%, "color-danger" <60%)
    pub fn get_health_css_class(&self) -> &str {
        if self.health_percent >= 80.0 {
            "color-success"
        } else if self.health_percent >= 60.0 {
            "color-warning"
        } else {
            "color-danger"
        }
    }

    /// Calculates power consumption in watts
    ///
    /// # Returns
    ///
    /// Power in watts (voltage × current)
    #[allow(clippy::cast_precision_loss)]
    pub fn power_watts(&self) -> f64 {
        (self.voltage_now as f64 / 1_000_000.0) * (self.current_now as f64 / 1_000_000.0)
    }

    /// Returns voltage in volts
    ///
    /// # Returns
    ///
    /// Voltage in V (converted from µV)
    #[allow(clippy::cast_precision_loss)]
    pub fn voltage_v(&self) -> f64 {
        self.voltage_now as f64 / 1_000_000.0
    }

    /// Returns current in milliamperes
    ///
    /// # Returns
    ///
    /// Current in mA (converted from µA)
    pub const fn current_ma(&self) -> u64 {
        self.current_now / 1000
    }

    /// Returns current charge in milliampere-hours
    ///
    /// # Returns
    ///
    /// Charge in mAh (converted from µAh)
    pub const fn charge_now_mah(&self) -> u64 {
        self.charge_now / 1000
    }

    /// Returns full charge capacity in milliampere-hours
    ///
    /// # Returns
    ///
    /// Full capacity in mAh (converted from µAh)
    pub const fn charge_full_mah(&self) -> u64 {
        self.charge_full / 1000
    }

    /// Returns design capacity in milliampere-hours
    ///
    /// # Returns
    ///
    /// Original design capacity in mAh (converted from µAh)
    pub const fn charge_full_design_mah(&self) -> u64 {
        self.charge_full_design / 1000
    }

    /// Returns formatted remaining time string
    ///
    /// # Returns
    ///
    /// * `Some(String)` - Time formatted as "Xh00 until full" or "Xh00 remaining"
    /// * `None` - Time cannot be calculated
    pub fn time_remaining_formatted(&self) -> Option<String> {
        self.time_remaining_minutes.map(|minutes| {
            let hours = minutes / 60;
            let mins = minutes % 60;
            if self.status == "Charging" {
                format!("⏱ {hours}h{mins:02} jusqu'à plein")
            } else {
                format!("⏱ {hours}h{mins:02} restant")
            }
        })
    }

    /// Returns alarm threshold as percentage of full capacity
    ///
    /// # Returns
    ///
    /// * `Some(f32)` - Alarm percentage
    /// * `None` - No alarm configured
    #[allow(clippy::cast_precision_loss)]
    pub fn alarm_percent(&self) -> Option<f32> {
        self.alarm
            .map(|a| (a as f32 / self.charge_full as f32) * 100.0)
    }

    /// Returns formatted systemd service status with markup
    ///
    /// # Returns
    ///
    /// Pango markup string (green "Active" or red "Inactive")
    pub fn service_status_markup(&self) -> String {
        if self.service_active {
            "<span size='xx-large' weight='bold'>Actif</span>".to_string()
        } else {
            "<span size='xx-large' weight='bold'>Inactif</span>".to_string()
        }
    }

    /// Returns CSS class for service status (active=success, inactive=danger)
    pub const fn service_status_css_class(&self) -> &str {
        if self.service_active {
            "color-success"
        } else {
            "color-danger"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_name_validation_valid() {
        // Les noms commençant par BAT sont valides (même si le fichier n'existe pas nécessairement)
        // Sur ce système, BAT1 existe, donc Ok() est retourné
        let result = BatteryInfo::new("BAT1");
        // Result can be Ok if battery exists, or Err(IoError) if it doesn't
        // but should never be Err(InvalidBatteryName)
        if let Err(e) = result {
            // If error, it must be IoError, not InvalidBatteryName
            match e {
                BatteryError::IoError(_) => {} // OK
                BatteryError::InvalidBatteryName(_) => panic!("BAT1 devrait être un nom valide"),
            }
        }
    }

    #[test]
    fn test_battery_name_validation_invalid() {
        // Noms invalides
        let result = BatteryInfo::new("AC0");
        assert!(result.is_err());
        match result {
            Err(BatteryError::InvalidBatteryName(name)) => {
                assert_eq!(name, "AC0");
            }
            _ => panic!("Should return InvalidBatteryName"),
        }

        assert!(BatteryInfo::new("invalid").is_err());
        assert!(BatteryInfo::new("").is_err());
        assert!(BatteryInfo::new("battery").is_err());
    }

    #[test]
    fn test_status_markup_format() {
        // Verify get_status_markup returns colored markup
        let info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Charging".to_string(),
            capacity_percent: 80,
            capacity_level: "Normal".to_string(),
            charge_now: 4_000_000,
            charge_full: 5_000_000,
            charge_full_design: 5_000_000,
            current_now: 1_000_000,
            voltage_now: 12_000_000,
            cycle_count: 50,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: Some(120),
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: false,
        };

        let markup = info.get_status_markup();
        assert!(markup.contains("<span"));
        assert!(!markup.contains("color=")); // Plus de couleurs inline
        assert!(markup.contains('⚡'));

        // Vérifier que la classe CSS est correcte
        assert_eq!(info.get_status_css_class(), "color-success");
    }

    #[test]
    fn test_health_calculation() {
        let mut info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test Model".to_string(),
            technology: "Li-ion".to_string(),
            status: "Discharging".to_string(),
            capacity_percent: 80,
            capacity_level: "Normal".to_string(),
            charge_now: 4_000_000,
            charge_full: 4_500_000,
            charge_full_design: 5_000_000,
            current_now: 500_000,
            voltage_now: 12_000_000,
            cycle_count: 100,
            health_percent: 0.0,
            wear_percent: 0.0,
            time_remaining_minutes: None,
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: false,
        };

        // Calcul manuel
        #[allow(clippy::cast_precision_loss)]
        let calculated_health = (info.charge_full as f32 / info.charge_full_design as f32) * 100.0;
        info.health_percent = calculated_health;
        info.wear_percent = 100.0 - info.health_percent;

        #[allow(clippy::float_cmp)]
        {
            assert_eq!(info.health_percent, 90.0);
            assert_eq!(info.wear_percent, 10.0);
        }
    }

    #[test]
    fn test_power_watts_calculation() {
        let info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Discharging".to_string(),
            capacity_percent: 80,
            capacity_level: "Normal".to_string(),
            charge_now: 4_000_000,
            charge_full: 5_000_000,
            charge_full_design: 5_000_000,
            current_now: 1_000_000,  // 1A
            voltage_now: 12_000_000, // 12V
            cycle_count: 50,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: None,
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: false,
        };

        let power = info.power_watts();
        assert!((power - 12.0).abs() < 0.01); // 12V * 1A = 12W
    }

    #[test]
    fn test_voltage_conversion() {
        let info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Full".to_string(),
            capacity_percent: 100,
            capacity_level: "Normal".to_string(),
            charge_now: 5_000_000,
            charge_full: 5_000_000,
            charge_full_design: 5_000_000,
            current_now: 0,
            voltage_now: 12_600_000, // 12.6V
            cycle_count: 10,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: None,
            charge_start_threshold: Some(60),
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: true,
        };

        #[allow(clippy::float_cmp)]
        {
            assert_eq!(info.voltage_v(), 12.6);
        }
    }

    #[test]
    fn test_current_conversion() {
        let info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Charging".to_string(),
            capacity_percent: 50,
            capacity_level: "Normal".to_string(),
            charge_now: 2_500_000,
            charge_full: 5_000_000,
            charge_full_design: 5_000_000,
            current_now: 2_500_000, // 2.5A = 2500mA
            voltage_now: 12_000_000,
            cycle_count: 25,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: Some(60),
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: Some(500_000),
            service_active: false,
        };

        assert_eq!(info.current_ma(), 2500);
    }

    #[test]
    fn test_charge_conversions() {
        let info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Discharging".to_string(),
            capacity_percent: 75,
            capacity_level: "Normal".to_string(),
            charge_now: 3_750_000,         // 3750 mAh
            charge_full: 5_000_000,        // 5000 mAh
            charge_full_design: 5_500_000, // 5500 mAh
            current_now: 500_000,
            voltage_now: 11_800_000,
            cycle_count: 150,
            health_percent: 90.9,
            wear_percent: 9.1,
            time_remaining_minutes: Some(450),
            charge_start_threshold: Some(40),
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: true,
        };

        assert_eq!(info.charge_now_mah(), 3750);
        assert_eq!(info.charge_full_mah(), 5000);
        assert_eq!(info.charge_full_design_mah(), 5500);
    }

    #[test]
    fn test_status_markup() {
        let mut info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Charging".to_string(),
            capacity_percent: 60,
            capacity_level: "Normal".to_string(),
            charge_now: 3_000_000,
            charge_full: 5_000_000,
            charge_full_design: 5_000_000,
            current_now: 1_000_000,
            voltage_now: 12_000_000,
            cycle_count: 50,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: Some(120),
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: false,
        };

        assert!(info.get_status_markup().contains('⚡'));

        info.status = "Discharging".to_string();
        assert!(info.get_status_markup().contains('🔋'));

        info.status = "Full".to_string();
        assert!(info.get_status_markup().contains('✓'));

        info.status = "Not charging".to_string();
        assert!(info.get_status_markup().contains('⏸'));

        info.capacity_percent = 100;
        assert!(info.get_status_markup().contains('✓'));

        info.status = "Unknown".to_string();
        assert!(info.get_status_markup().contains('?'));
    }

    #[test]
    fn test_alarm_percent() {
        let info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Discharging".to_string(),
            capacity_percent: 50,
            capacity_level: "Normal".to_string(),
            charge_now: 2_500_000,
            charge_full: 5_000_000,
            charge_full_design: 5_000_000,
            current_now: 500_000,
            voltage_now: 11_500_000,
            cycle_count: 100,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: Some(300),
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: Some(500_000), // 500000 µAh = 10% de 5000000
            service_active: false,
        };

        let alarm_pct = info.alarm_percent().unwrap();
        assert!((alarm_pct - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_service_status_markup() {
        let mut info = BatteryInfo {
            name: "BAT0".to_string(),
            manufacturer: "Test".to_string(),
            model_name: "Test".to_string(),
            technology: "Li-ion".to_string(),
            status: "Full".to_string(),
            capacity_percent: 100,
            capacity_level: "Full".to_string(),
            charge_now: 5_000_000,
            charge_full: 5_000_000,
            charge_full_design: 5_000_000,
            current_now: 0,
            voltage_now: 12_600_000,
            cycle_count: 5,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: None,
            charge_start_threshold: Some(60),
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: true,
        };

        assert!(info.service_status_markup().contains("Actif"));

        info.service_active = false;
        assert!(info.service_status_markup().contains("Inactif"));
    }
}
