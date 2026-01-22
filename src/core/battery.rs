use std::fs;

/// Constantes pour les formats markup
const MARKUP_LARGE_GREEN: &str = "<span size='xx-large' weight='bold' color='green'>{}</span>";
const MARKUP_LARGE_BLUE: &str = "<span size='xx-large' weight='bold' color='blue'>{}</span>";
const MARKUP_LARGE_ORANGE: &str = "<span size='xx-large' weight='bold' color='orange'>{}</span>";
const MARKUP_LARGE_RED: &str = "<span size='xx-large' weight='bold' color='red'>{}</span>";

/// Erreurs possibles lors de la création de BatteryInfo
#[derive(Debug)]
pub enum BatteryError {
    InvalidBatteryName(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for BatteryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryError::InvalidBatteryName(name) => {
                write!(f, "Nom de batterie invalide: '{}'. Le nom doit commencer par 'BAT'.", name)
            }
            BatteryError::IoError(e) => write!(f, "Erreur I/O: {}", e),
        }
    }
}

impl std::error::Error for BatteryError {}

impl From<std::io::Error> for BatteryError {
    fn from(error: std::io::Error) -> Self {
        BatteryError::IoError(error)
    }
}

/// Informations détaillées sur une batterie
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
    /// Crée une nouvelle instance `BatteryInfo` en lisant les fichiers système.
    /// 
    /// # Erreurs
    /// 
    /// Retourne `BatteryError::InvalidBatteryName` si le nom ne commence pas par "BAT".
    pub fn new(battery_name: &str) -> Result<Self, BatteryError> {
        // Validation du nom de batterie
        if !battery_name.starts_with("BAT") {
            return Err(BatteryError::InvalidBatteryName(battery_name.to_string()));
        }
        let base_path = format!("/sys/class/power_supply/{}", battery_name);

        let name = battery_name.to_string();

        let manufacturer = Self::read_sys_file(&format!("{}/manufacturer", base_path))
            .unwrap_or_else(|| "Inconnu".to_string());
        let model_name = Self::read_sys_file(&format!("{}/model_name", base_path))
            .unwrap_or_else(|| "Inconnu".to_string());
        let technology = Self::read_sys_file(&format!("{}/technology", base_path))
            .unwrap_or_else(|| "Inconnu".to_string());
        let status = Self::read_sys_file(&format!("{}/status", base_path))
            .unwrap_or_else(|| "Inconnu".to_string());
        let capacity_level = Self::read_sys_file(&format!("{}/capacity_level", base_path))
            .unwrap_or_else(|| "Inconnu".to_string());

        let capacity_percent = Self::read_sys_file(&format!("{}/capacity", base_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let charge_now = Self::read_sys_file(&format!("{}/charge_now", base_path))
            .or_else(|| Self::read_sys_file(&format!("{}/energy_now", base_path)))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let charge_full = Self::read_sys_file(&format!("{}/charge_full", base_path))
            .or_else(|| Self::read_sys_file(&format!("{}/energy_full", base_path)))
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        let charge_full_design = Self::read_sys_file(&format!("{}/charge_full_design", base_path))
            .or_else(|| Self::read_sys_file(&format!("{}/energy_full_design", base_path)))
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        let current_now = Self::read_sys_file(&format!("{}/current_now", base_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let voltage_now = Self::read_sys_file(&format!("{}/voltage_now", base_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let cycle_count = Self::read_sys_file(&format!("{}/cycle_count", base_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let health_percent = if charge_full_design > 0 {
            (charge_full as f32 / charge_full_design as f32) * 100.0
        } else {
            100.0
        };

        let wear_percent = 100.0 - health_percent;

        let time_remaining_minutes = if current_now > 0 && status == "Discharging" {
            Some((charge_now as f32 / current_now as f32 * 60.0) as u32)
        } else if current_now > 0 && status == "Charging" {
            Some(((charge_full - charge_now) as f32 / current_now as f32 * 60.0) as u32)
        } else {
            None
        };

        let charge_start_threshold =
            Self::read_sys_file(&format!("{}/charge_start_threshold", base_path))
                .or_else(|| {
                    Self::read_sys_file(&format!("{}/charge_control_start_threshold", base_path))
                })
                .and_then(|s| s.parse().ok());

        let charge_stop_threshold =
            Self::read_sys_file(&format!("{}/charge_stop_threshold", base_path))
                .or_else(|| {
                    Self::read_sys_file(&format!("{}/charge_control_end_threshold", base_path))
                })
                .and_then(|s| s.parse().ok());

        let alarm =
            Self::read_sys_file(&format!("{}/alarm", base_path)).and_then(|s| s.parse().ok());

        // Vérifier si le service systemd battery-manager est actif
        let service_active = std::process::Command::new("systemctl")
            .args(["is-active", "battery-manager.service"])
            .output()
            .ok()
            .map(|output| output.status.success())
            .unwrap_or(false);

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

    /// Lit un fichier système et retourne son contenu trimé
    fn read_sys_file(path: &str) -> Option<String> {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    }

    /// Retourne la liste des batteries disponibles
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
        batteries
    }

    /// Retourne le texte d'état formaté pour l'affichage
    pub fn get_status_markup(&self) -> String {
        match self.status.as_str() {
            "Charging" => MARKUP_LARGE_GREEN.replace("{}", "⚡ En charge"),
            "Discharging" => MARKUP_LARGE_ORANGE.replace("{}", "🔋 Décharge"),
            "Full" => MARKUP_LARGE_BLUE.replace("{}", "✓ Pleine"),
            "Not charging" => MARKUP_LARGE_ORANGE.replace("{}", "⏸ Pas en charge"),
            _ => MARKUP_LARGE_ORANGE.replace("{}", "? Inconnu"),
        }
    }

    /// Calcule la puissance en watts
    pub fn power_watts(&self) -> f64 {
        (self.voltage_now as f64 / 1_000_000.0) * (self.current_now as f64 / 1_000_000.0)
    }

    /// Retourne la tension en volts
    pub fn voltage_v(&self) -> f64 {
        self.voltage_now as f64 / 1_000_000.0
    }

    /// Retourne le courant en mA
    pub fn current_ma(&self) -> u64 {
        self.current_now / 1000
    }

    /// Retourne la charge actuelle en mAh
    pub fn charge_now_mah(&self) -> u64 {
        self.charge_now / 1000
    }

    /// Retourne la capacité max en mAh
    pub fn charge_full_mah(&self) -> u64 {
        self.charge_full / 1000
    }

    /// Retourne la capacité d'origine en mAh
    pub fn charge_full_design_mah(&self) -> u64 {
        self.charge_full_design / 1000
    }

    /// Retourne le temps restant formaté
    pub fn time_remaining_formatted(&self) -> Option<String> {
        self.time_remaining_minutes.map(|minutes| {
            let hours = minutes / 60;
            let mins = minutes % 60;
            if self.status == "Charging" {
                format!("⏱ {}h{:02} jusqu'à plein", hours, mins)
            } else {
                format!("⏱ {}h{:02} restant", hours, mins)
            }
        })
    }

    /// Retourne le pourcentage d'alarme par rapport à la capacité max
    pub fn alarm_percent(&self) -> Option<f32> {
        self.alarm
            .map(|a| (a as f32 / self.charge_full as f32) * 100.0)
    }

    /// Retourne l'état du service systemd formaté
    pub fn service_status_markup(&self) -> String {
        if self.service_active {
            MARKUP_LARGE_GREEN.replace("{}", "Actif")
        } else {
            MARKUP_LARGE_RED.replace("{}", "Inactif")
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
        // Le résultat peut être Ok si la batterie existe, ou Err(IoError) si elle n'existe pas
        // mais ne devrait jamais être Err(InvalidBatteryName)
        if let Err(e) = result {
            // Si erreur, ce doit être IoError, pas InvalidBatteryName
            match e {
                BatteryError::IoError(_) => {}, // OK
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
            _ => panic!("Devrait retourner InvalidBatteryName"),
        }

        assert!(BatteryInfo::new("invalid").is_err());
        assert!(BatteryInfo::new("").is_err());
        assert!(BatteryInfo::new("battery").is_err());
    }

    #[test]
    fn test_markup_constants() {
        // Vérifier que les constantes sont bien définies
        assert!(MARKUP_LARGE_GREEN.contains("green"));
        assert!(MARKUP_LARGE_BLUE.contains("blue"));
        assert!(MARKUP_LARGE_ORANGE.contains("orange"));
        assert!(MARKUP_LARGE_RED.contains("red"));
        assert!(MARKUP_LARGE_GREEN.contains("{}"));
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
            charge_now: 4000000,
            charge_full: 4500000,
            charge_full_design: 5000000,
            current_now: 500000,
            voltage_now: 12000000,
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
        info.health_percent = (info.charge_full as f32 / info.charge_full_design as f32) * 100.0;
        info.wear_percent = 100.0 - info.health_percent;

        assert_eq!(info.health_percent, 90.0);
        assert_eq!(info.wear_percent, 10.0);
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
            charge_now: 4000000,
            charge_full: 5000000,
            charge_full_design: 5000000,
            current_now: 1000000, // 1A
            voltage_now: 12000000, // 12V
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
            charge_now: 5000000,
            charge_full: 5000000,
            charge_full_design: 5000000,
            current_now: 0,
            voltage_now: 12600000, // 12.6V
            cycle_count: 10,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: None,
            charge_start_threshold: Some(60),
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: true,
        };

        assert_eq!(info.voltage_v(), 12.6);
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
            charge_now: 2500000,
            charge_full: 5000000,
            charge_full_design: 5000000,
            current_now: 2500000, // 2.5A = 2500mA
            voltage_now: 12000000,
            cycle_count: 25,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: Some(60),
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: Some(500000),
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
            charge_now: 3750000, // 3750 mAh
            charge_full: 5000000, // 5000 mAh
            charge_full_design: 5500000, // 5500 mAh
            current_now: 500000,
            voltage_now: 11800000,
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
            charge_now: 3000000,
            charge_full: 5000000,
            charge_full_design: 5000000,
            current_now: 1000000,
            voltage_now: 12000000,
            cycle_count: 50,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: Some(120),
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: None,
            service_active: false,
        };

        assert!(info.get_status_markup().contains("En charge"));
        
        info.status = "Discharging".to_string();
        assert!(info.get_status_markup().contains("Décharge"));
        
        info.status = "Full".to_string();
        assert!(info.get_status_markup().contains("Pleine"));
        
        info.status = "Not charging".to_string();
        assert!(info.get_status_markup().contains("Pas en charge"));
        
        info.status = "Unknown".to_string();
        assert!(info.get_status_markup().contains("Inconnu"));
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
            charge_now: 2500000,
            charge_full: 5000000,
            charge_full_design: 5000000,
            current_now: 500000,
            voltage_now: 11500000,
            cycle_count: 100,
            health_percent: 100.0,
            wear_percent: 0.0,
            time_remaining_minutes: Some(300),
            charge_start_threshold: None,
            charge_stop_threshold: Some(80),
            alarm: Some(500000), // 500000 µAh = 10% de 5000000
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
            charge_now: 5000000,
            charge_full: 5000000,
            charge_full_design: 5000000,
            current_now: 0,
            voltage_now: 12600000,
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
