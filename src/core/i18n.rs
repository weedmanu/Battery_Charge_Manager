//! Internationalization module for Battery Manager
//!
//! Provides translation support for English and French languages.
//! The language is set at runtime via command-line argument.

use std::collections::HashMap;
use std::sync::RwLock;

/// Current language setting (default: "fr")
static CURRENT_LANG: std::sync::LazyLock<RwLock<String>> =
    std::sync::LazyLock::new(|| RwLock::new("fr".to_string()));

/// Translation dictionary
static TRANSLATIONS: std::sync::LazyLock<
    HashMap<&'static str, HashMap<&'static str, &'static str>>,
> = std::sync::LazyLock::new(|| {
    let mut map = HashMap::new();

    // French translations
    let mut fr = HashMap::new();
    fr.insert("app_title", "Gestionnaire de Batterie");
    fr.insert("info_tab", "📊 Informations");
    fr.insert("settings_tab", "⚙️ Réglages");

    // Info tab
    fr.insert("power_source", "🔌 Source d'alimentation");
    fr.insert("on_ac", "Sur secteur");
    fr.insert("on_battery", "Sur batterie");
    fr.insert("battery_status", "⚡ État de la batterie");
    fr.insert("charging", "En charge");
    fr.insert("discharging", "En décharge");
    fr.insert("full", "Pleine");
    fr.insert("not_charging", "Ne charge pas");
    fr.insert("unknown", "Inconnu");
    fr.insert("charge_level", "🔋 Niveau de charge");
    fr.insert("battery_health", "💚 Santé de la batterie");
    fr.insert("electrical_params", "⚡ Paramètres électriques");
    fr.insert("voltage", "Tension");
    fr.insert("capacity", "Capacité");
    fr.insert("capacity_level", "Niveau");
    fr.insert("status", "Statut");
    fr.insert("connection", "Connexion");
    fr.insert("current", "Courant");
    fr.insert("power", "Puissance");
    fr.insert("system_info", "🖥️ Informations système");
    fr.insert("manufacturer", "Fabricant");
    fr.insert("model", "Modèle");
    fr.insert("technology", "Technologie");
    fr.insert("capacity_info", "📊 Informations de capacité");
    fr.insert("current_cap", "Actuelle");
    fr.insert("design_cap", "Nominale");
    fr.insert("charge_thresholds", "🎯 Seuils de charge");
    fr.insert("start_threshold", "Début");
    fr.insert("stop_threshold", "Fin");
    fr.insert("discharge_alarm", "⚠️ Alarme de décharge");
    fr.insert("systemd_service", "🔧 Service systemd");
    fr.insert("service_active", "Actif");
    fr.insert("service_inactive", "Inactif");

    // Settings tab
    fr.insert("vendor_info", "🏭 Informations du Système");
    fr.insert("product_name", "Modèle");
    fr.insert("start_support", "Seuil de début");
    fr.insert("stop_support", "Seuil de fin");
    fr.insert("charge_settings", "⚙️ Seuils de charge");
    fr.insert("start_threshold_pct", "Seuil de début (%)");
    fr.insert("stop_threshold_pct", "Seuil de fin de charge (%)");
    fr.insert("alarm_settings", "⚠️ Alarme de décharge");
    fr.insert("alarm_threshold", "Seuil d'alarme (%)");
    fr.insert("service_settings", "🔧 Service systemd");
    fr.insert(
        "enable_service",
        "Activer la restauration automatique au démarrage",
    );
    fr.insert("charge_100", "Charger à 100%");
    fr.insert(
        "settings_applied",
        "✓ Réglages appliqués (redémarrage requis)",
    );
    fr.insert("alarm", "Alarme");
    fr.insert("service", "Service");
    fr.insert("enabled", "activé");
    fr.insert("disabled", "désactivé");
    fr.insert("error", "Erreur");
    fr.insert("exec_error", "Erreur d'exécution");
    fr.insert("auth_canceled", "Authentification annulée");
    fr.insert("no_battery", "Aucune batterie détectée sur ce système");
    fr.insert(
        "error_battery_init",
        "Erreur lors de la création de BatteryInfo",
    );
    fr.insert("tab_info", "Informations");
    fr.insert("tab_settings", "Réglages");
    fr.insert("tab_ui", "Interface");
    fr.insert("tab_peripherals", "Périphériques");
    fr.insert("card_thresholds", "Seuils");
    fr.insert("card_charge", "Charge");
    fr.insert("card_health", "Santé");
    fr.insert("card_power", "Alimentation");
    fr.insert("card_status", "État");
    fr.insert("card_battery", "Batterie");
    fr.insert("card_electrical", "Électrique");
    fr.insert("card_capacity", "Capacité");
    fr.insert("card_service", "Service");
    fr.insert("card_peripherals", "Périphérique");
    fr.insert("card_info", "Informations");
    fr.insert("card_battery_status", "État Batterie");
    fr.insert("card_system_info", "Informations du Système");
    fr.insert("card_threshold_settings", "Seuils de charge");
    fr.insert("card_service_manager", "Service Battery Manager");
    fr.insert("threshold_start", "Début de charge");
    fr.insert("threshold_stop", "Fin de charge");
    fr.insert("threshold_start_pct", "Seuil de début (%)");
    fr.insert("threshold_stop_pct", "Seuil de fin de charge (%)");
    fr.insert("connected", "✓ Connecté");
    fr.insert("disconnected", "✗ Déconnecté");
    fr.insert("device_type", "Type");
    fr.insert("device_scope", "Portée");
    fr.insert("serial_number", "N° Série");
    fr.insert("wear", "Usure");
    fr.insert("cycles", "Cycles");
    fr.insert("adapter", "Adaptateur");
    fr.insert("name", "Nom");
    fr.insert("type", "Type");
    fr.insert("current_capacity", "Actuelle");
    fr.insert("full_capacity", "Complète");
    fr.insert("design_capacity", "Design");
    fr.insert("enable_systemd_service", "Activer le service systemd");
    fr.insert(
        "note_enabled",
        "<b>Activé :</b> applique les seuils immédiatement et de façon persistante",
    );
    fr.insert(
        "note_disabled",
        "<b>Désactivé :</b> applique les seuils immédiatement, mais ils seront perdus au prochain redémarrage",
    );
    fr.insert(
        "note_apply_required",
        "<b>Important :</b> les réglages sont pris en compte uniquement après avoir cliqué sur le bouton <i>Appliquer</i>.",
    );
    fr.insert(
        "warning_not_persistent",
        "⚠️ Sans service, ces réglages seront perdus au prochain redémarrage.",
    );
    fr.insert("apply_all_settings", "Appliquer tous les réglages");
    fr.insert(
        "error_start_greater_stop",
        "Erreur: le seuil de début doit être inférieur au seuil de fin",
    );
    fr.insert("success_applied", "Réglages appliqués avec succès");
    fr.insert("error_execution", "Erreur lors de l'exécution");
    fr.insert("language_setting", "Langue de l'interface");
    fr.insert("language_fr", "Français");
    fr.insert("language_en", "English");
    fr.insert(
        "language_changed",
        "Langue modifiée. Redémarrez l'application pour appliquer le changement.",
    );
    fr.insert(
        "restart_required",
        "Redémarrage automatique dans 1 seconde...",
    );
    fr.insert("theme_setting", "Thème de l'interface");
    fr.insert("theme_light", "Clair");
    fr.insert("theme_dark", "Sombre");
    fr.insert("theme_applied", "Thème appliqué immédiatement");
    fr.insert("not_detected", "Non détecté");
    fr.insert("time_until_full", "jusqu'à plein");
    fr.insert("time_remaining", "restant");

    // Documentation
    fr.insert("documentation", "Documentation");
    fr.insert("open_readme", "Ouvrir le README");
    fr.insert("open_references", "Ouvrir les références");
    fr.insert(
        "docs_not_found",
        "Documentation introuvable (non installée ?)",
    );
    fr.insert("docs_open_failed", "Impossible d'ouvrir la documentation");
    fr.insert("help", "Aide");

    // About / Help
    fr.insert("about", "À propos");
    fr.insert("open_about", "Ouvrir À propos");
    fr.insert(
        "about_text",
        "Gestionnaire de seuils de charge batterie (GTK4) avec restauration systemd.",
    );

    map.insert("fr", fr);

    // English translations
    let mut en = HashMap::new();
    en.insert("app_title", "Battery Manager");
    en.insert("info_tab", "📊 Information");
    en.insert("settings_tab", "⚙️ Settings");

    // Info tab
    en.insert("power_source", "🔌 Power Source");
    en.insert("on_ac", "On AC Power");
    en.insert("on_battery", "On Battery");
    en.insert("battery_status", "⚡ Battery Status");
    en.insert("charging", "Charging");
    en.insert("discharging", "Discharging");
    en.insert("full", "Full");
    en.insert("not_charging", "Not charging");
    en.insert("unknown", "Unknown");
    en.insert("charge_level", "🔋 Charge Level");
    en.insert("battery_health", "💚 Battery Health");
    en.insert("electrical_params", "⚡ Electrical Parameters");
    en.insert("voltage", "Voltage");
    en.insert("capacity", "Capacity");
    en.insert("capacity_level", "Level");
    en.insert("status", "Status");
    en.insert("connection", "Connection");
    en.insert("current", "Current");
    en.insert("power", "Power");
    en.insert("system_info", "🖥️ System Information");
    en.insert("manufacturer", "Manufacturer");
    en.insert("model", "Model");
    en.insert("technology", "Technology");
    en.insert("capacity_info", "📊 Capacity Information");
    en.insert("current_cap", "Current");
    en.insert("design_cap", "Design");
    en.insert("charge_thresholds", "🎯 Charge Thresholds");
    en.insert("start_threshold", "Start");
    en.insert("stop_threshold", "Stop");
    en.insert("discharge_alarm", "⚠️ Discharge Alarm");
    en.insert("systemd_service", "🔧 Systemd Service");
    en.insert("service_active", "Active");
    en.insert("service_inactive", "Inactive");

    // Settings tab
    en.insert("vendor_info", "🏭 System Information");
    en.insert("product_name", "Model");
    en.insert("start_support", "Start threshold");
    en.insert("stop_support", "Stop threshold");
    en.insert("charge_settings", "⚙️ Charge Thresholds");
    en.insert("start_threshold_pct", "Start threshold (%)");
    en.insert("stop_threshold_pct", "Stop threshold (%)");
    en.insert("alarm_settings", "⚠️ Discharge Alarm");
    en.insert("alarm_threshold", "Alarm threshold (%)");
    en.insert("service_settings", "🔧 Systemd Service");
    en.insert("enable_service", "Enable automatic restoration at boot");
    en.insert("charge_100", "Charge to 100%");
    en.insert("settings_applied", "✓ Settings applied (reboot required)");
    en.insert("alarm", "Alarm");
    en.insert("service", "Service");
    en.insert("enabled", "enabled");
    en.insert("disabled", "disabled");
    en.insert("error", "Error");
    en.insert("exec_error", "Execution error");
    en.insert("auth_canceled", "Authentication canceled");
    en.insert("no_battery", "No battery detected on this system");
    en.insert("error_battery_init", "Error creating BatteryInfo");
    en.insert("tab_info", "Information");
    en.insert("tab_settings", "Settings");
    en.insert("tab_ui", "Interface");
    en.insert("tab_peripherals", "Peripherals");
    en.insert("card_thresholds", "Thresholds");
    en.insert("card_charge", "Charge");
    en.insert("card_health", "Health");
    en.insert("card_power", "Power");
    en.insert("card_status", "Status");
    en.insert("card_battery", "Battery");
    en.insert("card_electrical", "Electrical");
    en.insert("card_capacity", "Capacity");
    en.insert("card_service", "Service");
    en.insert("card_peripherals", "Peripheral");
    en.insert("card_info", "Information");
    en.insert("card_battery_status", "Battery Status");
    en.insert("card_system_info", "System Information");
    en.insert("card_threshold_settings", "Charge Thresholds");
    en.insert("card_service_manager", "Battery Manager Service");
    en.insert("threshold_start", "Charge start");
    en.insert("threshold_stop", "Charge stop");
    en.insert("threshold_start_pct", "Start threshold (%)");
    en.insert("threshold_stop_pct", "Stop threshold (%)");
    en.insert("connected", "✓ Connected");
    en.insert("disconnected", "✗ Disconnected");
    en.insert("device_type", "Type");
    en.insert("device_scope", "Scope");
    en.insert("serial_number", "Serial");
    en.insert("wear", "Wear");
    en.insert("cycles", "Cycles");
    en.insert("adapter", "Adapter");
    en.insert("name", "Name");
    en.insert("type", "Type");
    en.insert("current_capacity", "Current");
    en.insert("full_capacity", "Full");
    en.insert("design_capacity", "Design");
    en.insert("enable_systemd_service", "Enable systemd service");
    en.insert(
        "note_enabled",
        "<b>Enabled:</b> applies thresholds immediately and persistently",
    );
    en.insert(
        "note_disabled",
        "<b>Disabled:</b> applies thresholds immediately, but changes will be lost after reboot",
    );
    en.insert(
        "note_apply_required",
        "<b>Important:</b> settings are applied only after clicking the <i>Apply</i> button.",
    );
    en.insert(
        "warning_not_persistent",
        "⚠️ Without the service, these settings will be lost after reboot.",
    );
    en.insert("apply_all_settings", "Apply all settings");
    en.insert(
        "error_start_greater_stop",
        "Error: start threshold must be lower than stop threshold",
    );
    en.insert("success_applied", "Settings applied successfully");
    en.insert("theme_setting", "Interface Theme");
    en.insert("theme_light", "Light");
    en.insert("theme_dark", "Dark");
    en.insert("theme_applied", "Theme applied immediately");
    en.insert("not_detected", "Not detected");
    en.insert("time_until_full", "until full");
    en.insert("time_remaining", "remaining");
    en.insert("tab_ui", "Interface");

    // Documentation
    en.insert("documentation", "Documentation");
    en.insert("help", "Help");
    en.insert("open_readme", "Open README");
    en.insert("open_references", "Open references");
    en.insert("docs_not_found", "Documentation not found (not installed?)");
    en.insert("docs_open_failed", "Unable to open documentation");

    // About / Help
    en.insert("about", "About");
    en.insert("open_about", "Open About");
    en.insert(
        "about_text",
        "Battery charge threshold manager (GTK4) with systemd restoration.",
    );
    en.insert("error_execution", "Execution error");
    en.insert("language_setting", "Interface Language");
    en.insert("language_fr", "Français");
    en.insert("language_en", "English");
    en.insert(
        "language_changed",
        "Language changed. Restart the application to apply.",
    );
    en.insert("restart_required", "Auto-restart in 1 second...");

    map.insert("en", en);

    map
});

/// Set the current language
///
/// # Arguments
/// * `lang` - Language code ("en" or "fr")
///
/// # Panics
/// Panics if the language `RwLock` is poisoned (indicates a serious bug in the application)
pub fn set_language(lang: &str) {
    let normalized = if lang == "en" { "en" } else { "fr" };

    if crate::core::debug::is_debug_enabled() {
        crate::core::debug::debug_log_args(std::format_args!(
            "🌐 [I18N] set_language -> {normalized}"
        ));
    }

    *CURRENT_LANG
        .write()
        .expect("Language RwLock poisoned - this is a critical bug") = normalized.to_string();
}

/// Get the current language
///
/// # Panics
/// Panics if the language `RwLock` is poisoned (indicates a serious bug in the application)
pub fn get_language() -> String {
    CURRENT_LANG
        .read()
        .expect("Language RwLock poisoned - this is a critical bug")
        .clone()
}

/// Get a translated string
///
/// # Arguments
/// * `key` - Translation key
///
/// # Returns
/// Translated string for current language, or the key itself if not found
///
/// # Panics
/// Panics if the language `RwLock` is poisoned (indicates a serious bug in the application)
pub fn t(key: &str) -> String {
    let lang = CURRENT_LANG
        .read()
        .expect("Language RwLock poisoned - this is a critical bug")
        .clone();
    TRANSLATIONS
        .get(lang.as_str())
        .and_then(|lang_map| lang_map.get(key))
        .map_or_else(|| key.to_string(), std::string::ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mutex pour synchroniser les tests qui modifient CURRENT_LANG
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_1_default_language() {
        let _lock = TEST_MUTEX.lock().unwrap();
        // Premier test - vérifie la langue par défaut
        assert_eq!(get_language(), "fr");
    }

    #[test]
    fn test_2_set_language() {
        let _lock = TEST_MUTEX.lock().unwrap();
        // Deuxième test - teste le changement de langue
        set_language("en");
        assert_eq!(get_language(), "en");
        set_language("fr");
        assert_eq!(get_language(), "fr");
    }

    #[test]
    fn test_3_translation_fr() {
        let _lock = TEST_MUTEX.lock().unwrap();
        // Troisième test - teste les traductions françaises
        set_language("fr");
        assert_eq!(get_language(), "fr");
        assert_eq!(t("app_title"), "Gestionnaire de Batterie");
        assert_eq!(t("charging"), "En charge");
    }

    #[test]
    fn test_4_translation_en() {
        let _lock = TEST_MUTEX.lock().unwrap();
        // Quatrième test - teste les traductions anglaises
        set_language("en");
        assert_eq!(get_language(), "en");
        assert_eq!(t("app_title"), "Battery Manager");
        assert_eq!(t("charging"), "Charging");
    }

    #[test]
    fn test_5_missing_key() {
        let _lock = TEST_MUTEX.lock().unwrap();
        // Cinquième test - teste les clés manquantes
        set_language("en");
        assert_eq!(t("non_existent_key"), "non_existent_key");
    }
}
