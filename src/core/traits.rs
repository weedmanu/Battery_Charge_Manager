//! Traits pour l'abstraction des services et injection de dépendances

use super::battery::{BatteryError, BatteryInfo};

/// Service de gestion des batteries
/// 
/// Permet d'abstraire la source des informations de batterie
/// pour faciliter les tests avec des mocks.
#[allow(dead_code)]
pub trait BatteryService {
    /// Récupère les informations d'une batterie spécifique
    /// 
    /// # Arguments
    /// 
    /// * `name` - Le nom de la batterie (ex: "BAT0", "BAT1")
    /// 
    /// # Erreurs
    /// 
    /// Retourne `BatteryError` si le nom est invalide ou si la lecture échoue
    fn get_info(&self, name: &str) -> Result<BatteryInfo, BatteryError>;
    
    /// Liste toutes les batteries disponibles sur le système
    fn list_batteries(&self) -> Vec<String>;
}

/// Implémentation réelle du service de batterie
#[allow(dead_code)]
pub struct SystemBatteryService;

impl BatteryService for SystemBatteryService {
    fn get_info(&self, name: &str) -> Result<BatteryInfo, BatteryError> {
        BatteryInfo::new(name)
    }
    
    fn list_batteries(&self) -> Vec<String> {
        BatteryInfo::get_battery_list()
    }
}

/// Service d'écriture des seuils de charge
/// 
/// Abstrait l'écriture des seuils pour permettre les tests
#[allow(dead_code)]
pub trait ThresholdWriter {
    /// Applique des seuils de charge à une batterie
    /// 
    /// # Arguments
    /// 
    /// * `battery` - Le nom de la batterie
    /// * `start` - Seuil de démarrage (0-100)
    /// * `stop` - Seuil d'arrêt (0-100)
    /// 
    /// # Erreurs
    /// 
    /// Retourne une erreur si l'application échoue
    fn apply_thresholds(&self, battery: &str, start: Option<u8>, stop: u8) -> Result<(), String>;
    
    /// Vérifie si les seuils de démarrage sont supportés
    fn supports_start_threshold(&self) -> bool;
}

/// Implémentation système du writer de seuils
#[allow(dead_code)]
pub struct SystemThresholdWriter {
    supports_start: bool,
}

impl SystemThresholdWriter {
    /// Crée un nouveau writer système
    #[allow(dead_code)]
    pub fn new(supports_start: bool) -> Self {
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
                return Err("Le seuil de démarrage doit être inférieur au seuil d'arrêt".to_string());
            }
        }
        
        // Note: L'écriture réelle est faite par pkexec dans settings_tab.rs
        // Ce trait sert principalement pour les tests et l'abstraction
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
            // Retourne une erreur IoError car le mock ne peut pas lire les vrais fichiers
            Err(BatteryError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Mock service"
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
