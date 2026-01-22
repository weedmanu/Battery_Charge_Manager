use std::fs;

/// Informations sur l'alimentation secteur
#[derive(Debug, Clone)]
pub struct PowerSupplyInfo {
    pub ac_online: bool,
    pub ac_name: String,
}

impl PowerSupplyInfo {
    /// Crée une nouvelle instance en détectant l'alimentation secteur
    pub fn new() -> Self {
        let mut ac_online = false;
        let mut ac_name = String::from("Non détecté");

        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let type_path = format!("/sys/class/power_supply/{}/type", name);
                if let Ok(psu_type) = fs::read_to_string(&type_path) {
                    if psu_type.trim() == "Mains" {
                        ac_name = name.clone();
                        let online_path = format!("/sys/class/power_supply/{}/online", name);
                        if let Ok(online) = fs::read_to_string(&online_path) {
                            ac_online = online.trim() == "1";
                        }
                        break;
                    }
                }
            }
        }

        Self { ac_online, ac_name }
    }

    /// Retourne le markup pour l'état de l'alimentation
    pub fn get_power_source_markup(&self) -> &'static str {
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
