use std::fs;

/// Détection du fabricant et méthodes de gestion de batterie
#[derive(Debug, Clone, PartialEq)]
pub enum VendorType {
    Asus,
    Lenovo, // ThinkPad
    Dell,
    Huawei,
    System76,
    Tuxedo,
    Samsung,
    Sony,
    Lg,
    Msi,
    Toshiba,
    Macbook,
    Generic,
}

#[derive(Debug, Clone)]
pub struct VendorInfo {
    pub manufacturer: String,
    pub product_name: String,
    pub supports_start_threshold: bool,
    pub supports_stop_threshold: bool,
}

#[derive(Debug, Clone)]
pub struct ThresholdFiles {
    pub start_paths: Vec<String>,
    pub stop_paths: Vec<String>,
}

impl VendorInfo {
    /// Détecte automatiquement le fabricant du système
    pub fn detect() -> Self {
        let manufacturer = Self::read_dmi("sys_vendor")
            .unwrap_or_else(|| "Unknown".to_string())
            .to_lowercase();

        let product = Self::read_dmi("product_name").unwrap_or_else(|| "Unknown".to_string());

        let vendor_type = Self::identify_vendor(&manufacturer, &product);
        let threshold_files = Self::get_threshold_files(&vendor_type);

        // Vérifier le support réel des fichiers
        let supports_start = threshold_files
            .start_paths
            .iter()
            .any(|p| fs::metadata(p).is_ok());
        let supports_stop = threshold_files
            .stop_paths
            .iter()
            .any(|p| fs::metadata(p).is_ok());

        VendorInfo {
            manufacturer: manufacturer.clone(),
            product_name: product,
            supports_start_threshold: supports_start,
            supports_stop_threshold: supports_stop,
        }
    }

    /// Lit les informations DMI du système
    fn read_dmi(field: &str) -> Option<String> {
        fs::read_to_string(format!("/sys/class/dmi/id/{}", field))
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Identifie le type de fabricant
    fn identify_vendor(manufacturer: &str, product: &str) -> VendorType {
        // ASUS
        if manufacturer.contains("asus") || manufacturer.contains("asustek") {
            return VendorType::Asus;
        }

        // Lenovo / ThinkPad
        if manufacturer.contains("lenovo") || product.to_lowercase().contains("thinkpad") {
            return VendorType::Lenovo;
        }

        // Dell
        if manufacturer.contains("dell") {
            return VendorType::Dell;
        }

        // Huawei
        if manufacturer.contains("huawei") {
            return VendorType::Huawei;
        }

        // System76
        if manufacturer.contains("system76") {
            return VendorType::System76;
        }

        // Tuxedo
        if manufacturer.contains("tuxedo") {
            return VendorType::Tuxedo;
        }

        // Samsung
        if manufacturer.contains("samsung") {
            return VendorType::Samsung;
        }

        // Sony
        if manufacturer.contains("sony") {
            return VendorType::Sony;
        }

        // LG
        if manufacturer.contains("lg") {
            return VendorType::Lg;
        }

        // MSI
        if manufacturer.contains("micro-star") || manufacturer.contains("msi") {
            return VendorType::Msi;
        }

        // Toshiba
        if manufacturer.contains("toshiba") {
            return VendorType::Toshiba;
        }

        // Apple / Macbook
        if manufacturer.contains("apple") {
            return VendorType::Macbook;
        }

        VendorType::Generic
    }

    /// Retourne les chemins de fichiers de seuils selon le fabricant
    fn get_threshold_files(vendor: &VendorType) -> ThresholdFiles {
        let battery_base = "/sys/class/power_supply";

        match vendor {
            VendorType::Asus => {
                // ASUS: généralement seul charge_control_end_threshold
                ThresholdFiles {
                    start_paths: vec![],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                        format!("{}/BATC/charge_control_end_threshold", battery_base),
                        format!("{}/BATT/charge_control_end_threshold", battery_base),
                    ],
                }
            }

            VendorType::Lenovo => {
                // ThinkPad: support complet start/stop (kernel 5.9+)
                ThresholdFiles {
                    start_paths: vec![
                        format!("{}/BAT0/charge_control_start_threshold", battery_base),
                        format!("{}/BAT1/charge_control_start_threshold", battery_base),
                        format!("{}/BAT0/charge_start_threshold", battery_base),
                        format!("{}/BAT1/charge_start_threshold", battery_base),
                    ],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                        format!("{}/BAT0/charge_stop_threshold", battery_base),
                        format!("{}/BAT1/charge_stop_threshold", battery_base),
                    ],
                }
            }

            VendorType::Dell => {
                // Dell: support start/stop (kernel 6.12+)
                ThresholdFiles {
                    start_paths: vec![
                        format!("{}/BAT0/charge_control_start_threshold", battery_base),
                        format!("{}/BAT1/charge_control_start_threshold", battery_base),
                    ],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                    ],
                }
            }

            VendorType::Huawei => {
                // Huawei: fichier unique combiné
                ThresholdFiles {
                    start_paths: vec![
                        "/sys/devices/platform/huawei-wmi/charge_control_thresholds".to_string()
                    ],
                    stop_paths: vec![
                        "/sys/devices/platform/huawei-wmi/charge_control_thresholds".to_string()
                    ],
                }
            }

            VendorType::System76 | VendorType::Tuxedo => {
                // System76 & Tuxedo: support start/stop
                ThresholdFiles {
                    start_paths: vec![
                        format!("{}/BAT0/charge_control_start_threshold", battery_base),
                        format!("{}/BAT1/charge_control_start_threshold", battery_base),
                    ],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                    ],
                }
            }

            VendorType::Samsung => {
                // Samsung: Battery Life Extender (0/1)
                ThresholdFiles {
                    start_paths: vec![],
                    stop_paths: vec![
                        format!("{}/BAT0/battery_care_limit", battery_base),
                        format!("{}/BAT1/battery_care_limit", battery_base),
                    ],
                }
            }

            VendorType::Sony => {
                // Sony: battery_care_limiter
                ThresholdFiles {
                    start_paths: vec![],
                    stop_paths: vec![
                        format!("{}/BAT0/battery_care_limiter", battery_base),
                        format!("{}/BAT1/battery_care_limiter", battery_base),
                    ],
                }
            }

            VendorType::Lg => {
                // LG: battery_care_limit
                ThresholdFiles {
                    start_paths: vec![],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                    ],
                }
            }

            VendorType::Msi => {
                // MSI: support start/stop
                ThresholdFiles {
                    start_paths: vec![
                        format!("{}/BAT0/charge_control_start_threshold", battery_base),
                        format!("{}/BAT1/charge_control_start_threshold", battery_base),
                    ],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                    ],
                }
            }

            VendorType::Toshiba => {
                // Toshiba: stop seulement
                ThresholdFiles {
                    start_paths: vec![],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                    ],
                }
            }

            VendorType::Macbook => {
                // Macbook: support start/stop (avec macsmc-battery)
                ThresholdFiles {
                    start_paths: vec![format!(
                        "{}/macsmc-battery/charge_control_start_threshold",
                        battery_base
                    )],
                    stop_paths: vec![format!(
                        "{}/macsmc-battery/charge_control_end_threshold",
                        battery_base
                    )],
                }
            }

            VendorType::Generic => {
                // Générique: cherche tous les chemins possibles
                ThresholdFiles {
                    start_paths: vec![
                        format!("{}/BAT0/charge_control_start_threshold", battery_base),
                        format!("{}/BAT1/charge_control_start_threshold", battery_base),
                        format!("{}/BAT0/charge_start_threshold", battery_base),
                        format!("{}/BAT1/charge_start_threshold", battery_base),
                    ],
                    stop_paths: vec![
                        format!("{}/BAT0/charge_control_end_threshold", battery_base),
                        format!("{}/BAT1/charge_control_end_threshold", battery_base),
                        format!("{}/BAT0/charge_stop_threshold", battery_base),
                        format!("{}/BAT1/charge_stop_threshold", battery_base),
                        format!("{}/BAT0/charge_end_threshold", battery_base),
                        format!("{}/BAT1/charge_end_threshold", battery_base),
                    ],
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_vendor_asus() {
        let vendor = VendorInfo::identify_vendor("asustek computer inc.", "TUF Gaming");
        assert_eq!(vendor, VendorType::Asus);

        let vendor2 = VendorInfo::identify_vendor("asus", "VivoBook");
        assert_eq!(vendor2, VendorType::Asus);
    }

    #[test]
    fn test_identify_vendor_lenovo() {
        let vendor = VendorInfo::identify_vendor("lenovo", "ThinkPad X1");
        assert_eq!(vendor, VendorType::Lenovo);

        let vendor2 = VendorInfo::identify_vendor("generic", "ThinkPad T480");
        assert_eq!(vendor2, VendorType::Lenovo);
    }

    #[test]
    fn test_identify_vendor_dell() {
        let vendor = VendorInfo::identify_vendor("dell inc.", "XPS 13");
        assert_eq!(vendor, VendorType::Dell);
    }

    #[test]
    fn test_identify_vendor_huawei() {
        let vendor = VendorInfo::identify_vendor("huawei", "MateBook X Pro");
        assert_eq!(vendor, VendorType::Huawei);
    }

    #[test]
    fn test_identify_vendor_system76() {
        let vendor = VendorInfo::identify_vendor("system76", "Lemur Pro");
        assert_eq!(vendor, VendorType::System76);
    }

    #[test]
    fn test_identify_vendor_tuxedo() {
        let vendor = VendorInfo::identify_vendor("tuxedo", "InfinityBook");
        assert_eq!(vendor, VendorType::Tuxedo);
    }

    #[test]
    fn test_identify_vendor_samsung() {
        let vendor = VendorInfo::identify_vendor("samsung", "Galaxy Book");
        assert_eq!(vendor, VendorType::Samsung);
    }

    #[test]
    fn test_identify_vendor_generic() {
        let vendor = VendorInfo::identify_vendor("unknown manufacturer", "unknown product");
        assert_eq!(vendor, VendorType::Generic);
    }

    #[test]
    fn test_threshold_files_asus() {
        let files = VendorInfo::get_threshold_files(&VendorType::Asus);
        assert!(files.start_paths.is_empty()); // ASUS: pas de start threshold
        assert!(!files.stop_paths.is_empty()); // ASUS: stop threshold supporté
    }

    #[test]
    fn test_threshold_files_lenovo() {
        let files = VendorInfo::get_threshold_files(&VendorType::Lenovo);
        assert!(!files.start_paths.is_empty()); // Lenovo: start threshold
        assert!(!files.stop_paths.is_empty()); // Lenovo: stop threshold
    }

    #[test]
    fn test_threshold_files_generic() {
        let files = VendorInfo::get_threshold_files(&VendorType::Generic);
        assert!(!files.start_paths.is_empty());
        assert!(!files.stop_paths.is_empty());
        // Vérifier que les chemins génériques sont présents
        assert!(files
            .stop_paths
            .iter()
            .any(|p| p.contains("charge_control_end_threshold")));
    }

    #[test]
    fn test_vendor_detection_returns_valid_info() {
        let info = VendorInfo::detect();
        // Vérifie que la détection retourne toujours quelque chose
        assert!(!info.manufacturer.is_empty());
        assert!(!info.product_name.is_empty());
        // Au moins un des deux devrait être supporté sur un système moderne
        // (ou les deux peuvent être false sur système sans support batterie)
    }
}
