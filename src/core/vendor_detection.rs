//! Vendor detection and battery threshold file discovery
//!
//! Identifies laptop manufacturer and locates charge threshold control files
//! in `/sys/class/power_supply/`.

use std::fs;

/// Laptop vendor types with different battery control interfaces
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Vendor-specific battery information
#[derive(Debug, Clone)]
pub struct VendorInfo {
    pub manufacturer: String,
    pub product_name: String,
    pub supports_start_threshold: bool,
    pub supports_stop_threshold: bool,
}

/// Battery charge threshold file paths
#[derive(Debug, Clone)]
pub struct ThresholdFiles {
    pub start_paths: Vec<String>,
    pub stop_paths: Vec<String>,
}

impl VendorInfo {
    /// Automatically detects system vendor and threshold support
    ///
    /// Reads DMI information and checks for threshold files existence
    ///
    /// # Returns
    ///
    /// `VendorInfo` with manufacturer, product name, and threshold support flags
    pub fn detect() -> Self {
        let manufacturer = Self::read_dmi("sys_vendor")
            .unwrap_or_else(|| "Unknown".to_string())
            .to_lowercase();

        let product = Self::read_dmi("product_name").unwrap_or_else(|| "Unknown".to_string());

        let vendor_type = Self::identify_vendor(&manufacturer, &product);
        let threshold_files = Self::get_threshold_files(&vendor_type);

        if crate::core::debug::is_debug_enabled() {
            crate::core::debug::debug_log_args(std::format_args!(
                "🏭 [VENDOR] manufacturer={manufacturer} product={product} vendor_type={vendor_type:?}"
            ));
        }

        // Vérifier le support réel des fichiers
        let supports_start = threshold_files
            .start_paths
            .iter()
            .any(|p| fs::metadata(p).is_ok());
        let supports_stop = threshold_files
            .stop_paths
            .iter()
            .any(|p| fs::metadata(p).is_ok());

        if crate::core::debug::is_debug_enabled() {
            crate::core::debug::debug_log_args(std::format_args!(
                "🎯 [VENDOR] supports_start={supports_start} supports_stop={supports_stop}"
            ));
        }

        Self {
            manufacturer,
            product_name: product,
            supports_start_threshold: supports_start,
            supports_stop_threshold: supports_stop,
        }
    }

    /// Reads DMI system information from `/sys/class/dmi/id/`
    ///
    /// # Arguments
    ///
    /// * `field` - DMI field name (e.g., "`sys_vendor`", "`product_name`")
    ///
    /// # Returns
    ///
    /// * `Some(String)` - DMI field value (trimmed)
    /// * `None` - Field doesn't exist or read error
    fn read_dmi(field: &str) -> Option<String> {
        fs::read_to_string(format!("/sys/class/dmi/id/{field}"))
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Identifies laptop vendor type from manufacturer and product strings
    ///
    /// # Arguments
    ///
    /// * `manufacturer` - Manufacturer string (e.g., "asus", "lenovo")
    /// * `product` - Product name string
    ///
    /// # Returns
    ///
    /// Corresponding `VendorType` enum variant
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

    /// Returns threshold file paths for the vendor type
    ///
    /// # Arguments
    ///
    /// * `vendor` - Laptop vendor type
    ///
    /// # Returns
    ///
    /// `ThresholdFiles` with start and stop paths for the vendor
    fn get_threshold_files(vendor: &VendorType) -> ThresholdFiles {
        let base = "/sys/class/power_supply";

        // Helper to generate battery paths
        let bat_paths = |batteries: &[&str], suffix: &str| {
            batteries
                .iter()
                .map(|b| format!("{base}/{b}/{suffix}"))
                .collect()
        };

        match vendor {
            VendorType::Asus => ThresholdFiles {
                start_paths: vec![],
                stop_paths: bat_paths(
                    &["BAT0", "BAT1", "BATC", "BATT"],
                    "charge_control_end_threshold",
                ),
            },
            VendorType::Lenovo => ThresholdFiles {
                start_paths: bat_paths(&["BAT0", "BAT1"], "charge_control_start_threshold")
                    .into_iter()
                    .chain(bat_paths(&["BAT0", "BAT1"], "charge_start_threshold"))
                    .collect(),
                stop_paths: bat_paths(&["BAT0", "BAT1"], "charge_control_end_threshold")
                    .into_iter()
                    .chain(bat_paths(&["BAT0", "BAT1"], "charge_stop_threshold"))
                    .collect(),
            },
            VendorType::Dell | VendorType::System76 | VendorType::Tuxedo | VendorType::Msi => {
                ThresholdFiles {
                    start_paths: bat_paths(&["BAT0", "BAT1"], "charge_control_start_threshold"),
                    stop_paths: bat_paths(&["BAT0", "BAT1"], "charge_control_end_threshold"),
                }
            }
            VendorType::Huawei => ThresholdFiles {
                start_paths: vec![
                    "/sys/devices/platform/huawei-wmi/charge_control_thresholds".to_string()
                ],
                stop_paths: vec![
                    "/sys/devices/platform/huawei-wmi/charge_control_thresholds".to_string()
                ],
            },
            VendorType::Samsung => ThresholdFiles {
                start_paths: vec![],
                stop_paths: bat_paths(&["BAT0", "BAT1"], "battery_care_limit"),
            },
            VendorType::Sony => ThresholdFiles {
                start_paths: vec![],
                stop_paths: bat_paths(&["BAT0", "BAT1"], "battery_care_limiter"),
            },
            VendorType::Lg | VendorType::Toshiba => ThresholdFiles {
                start_paths: vec![],
                stop_paths: bat_paths(&["BAT0", "BAT1"], "charge_control_end_threshold"),
            },
            VendorType::Macbook => ThresholdFiles {
                start_paths: vec![format!(
                    "{base}/macsmc-battery/charge_control_start_threshold"
                )],
                stop_paths: vec![format!(
                    "{base}/macsmc-battery/charge_control_end_threshold"
                )],
            },
            VendorType::Generic => ThresholdFiles {
                start_paths: bat_paths(&["BAT0", "BAT1"], "charge_control_start_threshold")
                    .into_iter()
                    .chain(bat_paths(&["BAT0", "BAT1"], "charge_start_threshold"))
                    .collect(),
                stop_paths: bat_paths(&["BAT0", "BAT1"], "charge_control_end_threshold")
                    .into_iter()
                    .chain(bat_paths(&["BAT0", "BAT1"], "charge_stop_threshold"))
                    .chain(bat_paths(&["BAT0", "BAT1"], "charge_end_threshold"))
                    .collect(),
            },
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
        assert!(files.start_paths.is_empty()); // ASUS: no start threshold
        assert!(!files.stop_paths.is_empty()); // ASUS: stop threshold supported
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
        // Verify detection always returns something
        assert!(!info.manufacturer.is_empty());
        assert!(!info.product_name.is_empty());
        // At least one should be supported on modern systems
        // (or both can be false on systems without battery support)
    }
}
