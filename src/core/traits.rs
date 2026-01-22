//! Trait abstractions for dependency injection and testing
//!
//! Provides `BatteryService` and `ThresholdWriter` traits to abstract
//! battery operations, enabling mock implementations for unit tests.

use super::battery::{BatteryError, BatteryInfo};

/// Battery information service trait
///
/// Abstracts battery data source for easier testing with mocks
pub trait BatteryService {
    /// Retrieves information for a specific battery
    ///
    /// # Arguments
    ///
    /// * `name` - Battery name (e.g., "BAT0", "BAT1")
    ///
    /// # Errors
    ///
    /// Returns `BatteryError` if name is invalid or read fails
    fn get_info(&self, name: &str) -> Result<BatteryInfo, BatteryError>;

    /// Lists all available batteries on the system
    fn list_batteries(&self) -> Vec<String>;
}

/// Real battery service implementation
pub struct SystemBatteryService;

impl BatteryService for SystemBatteryService {
    fn get_info(&self, name: &str) -> Result<BatteryInfo, BatteryError> {
        BatteryInfo::new(name)
    }

    fn list_batteries(&self) -> Vec<String> {
        BatteryInfo::get_battery_list()
    }
}

/// Charge threshold writer service trait
///
/// Abstracts threshold writing for testing purposes
pub trait ThresholdWriter {
    /// Applies charge thresholds to a battery
    ///
    /// # Arguments
    ///
    /// * `battery` - Battery name
    /// * `start` - Start threshold (0-100), if supported
    /// * `stop` - Stop threshold (0-100)
    ///
    /// # Errors
    ///
    /// Returns error if application fails
    fn apply_thresholds(&self, battery: &str, start: Option<u8>, stop: u8) -> Result<(), String>;

    /// Checks if start threshold is supported
    fn supports_start_threshold(&self) -> bool;
}

/// System threshold writer implementation
pub struct SystemThresholdWriter {
    supports_start: bool,
}

impl SystemThresholdWriter {
    /// Creates a new system threshold writer
    pub const fn new(supports_start: bool) -> Self {
        Self { supports_start }
    }
}

impl ThresholdWriter for SystemThresholdWriter {
    fn apply_thresholds(&self, _battery: &str, start: Option<u8>, stop: u8) -> Result<(), String> {
        // Validation
        if stop > 100 {
            return Err("Seuil d'arrêt invalide (> 100)".to_string());
        }

        if let Some(s) = start {
            if s > 100 {
                return Err("Seuil de démarrage invalide (> 100)".to_string());
            }
            if s >= stop {
                return Err(
                    "Le seuil de démarrage doit être inférieur au seuil d'arrêt".to_string()
                );
            }
        }

        // Note: Actual writing is done by pkexec in settings_tab.rs
        // This trait is mainly for tests and abstraction
        Ok(())
    }

    fn supports_start_threshold(&self) -> bool {
        self.supports_start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock du service de batterie pour les tests
    struct MockBatteryService {
        batteries: Vec<String>,
    }

    impl MockBatteryService {
        fn new(batteries: Vec<String>) -> Self {
            Self { batteries }
        }
    }

    impl BatteryService for MockBatteryService {
        fn get_info(&self, name: &str) -> Result<BatteryInfo, BatteryError> {
            if !name.starts_with("BAT") {
                return Err(BatteryError::InvalidBatteryName(name.to_string()));
            }
            // Returns IoError as mock service cannot read real files
            Err(BatteryError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Mock service",
            )))
        }

        fn list_batteries(&self) -> Vec<String> {
            self.batteries.clone()
        }
    }

    #[test]
    fn test_mock_battery_service() {
        let mock = MockBatteryService::new(vec!["BAT0".to_string(), "BAT1".to_string()]);
        let batteries = mock.list_batteries();
        assert_eq!(batteries.len(), 2);
        assert_eq!(batteries[0], "BAT0");
        assert_eq!(batteries[1], "BAT1");
    }

    #[test]
    fn test_mock_battery_service_validation() {
        let mock = MockBatteryService::new(vec![]);
        let result = mock.get_info("INVALID");
        assert!(result.is_err());
        match result {
            Err(BatteryError::InvalidBatteryName(name)) => assert_eq!(name, "INVALID"),
            _ => panic!("Expected InvalidBatteryName error"),
        }
    }

    #[test]
    fn test_system_battery_service_list() {
        let service = SystemBatteryService;
        let batteries = service.list_batteries();
        // Au moins une batterie devrait être présente sur le système de test
        assert!(!batteries.is_empty());
    }

    #[test]
    fn test_threshold_writer_validation() {
        let writer = SystemThresholdWriter::new(true);

        // Test seuil stop > 100
        let result = writer.apply_thresholds("BAT0", None, 150);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalide"));

        // Test seuil start > 100
        let result = writer.apply_thresholds("BAT0", Some(150), 80);
        assert!(result.is_err());

        // Test start >= stop
        let result = writer.apply_thresholds("BAT0", Some(80), 80);
        assert!(result.is_err());

        let result = writer.apply_thresholds("BAT0", Some(85), 80);
        assert!(result.is_err());
    }

    #[test]
    fn test_threshold_writer_valid() {
        let writer = SystemThresholdWriter::new(true);

        // Seuils valides
        let result = writer.apply_thresholds("BAT0", Some(60), 80);
        assert!(result.is_ok());

        let result = writer.apply_thresholds("BAT0", None, 80);
        assert!(result.is_ok());
    }

    #[test]
    fn test_threshold_writer_supports_start() {
        let writer_with_start = SystemThresholdWriter::new(true);
        assert!(writer_with_start.supports_start_threshold());

        let writer_without_start = SystemThresholdWriter::new(false);
        assert!(!writer_without_start.supports_start_threshold());
    }
}
