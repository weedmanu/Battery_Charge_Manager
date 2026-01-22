//! Core business logic for battery management
//!
//! Provides battery information reading, vendor detection, threshold
//! management, power supply monitoring, internationalization, and debug logging.

pub mod battery;
pub mod debug;
pub mod i18n;
pub mod power_supply;
pub mod traits;
pub mod vendor_detection;

pub use battery::BatteryInfo;
pub use power_supply::PowerSupplyInfo;
pub use vendor_detection::VendorInfo;

/// Exported traits for dependency injection and testing
#[allow(unused_imports)]
pub use traits::{BatteryService, SystemBatteryService, SystemThresholdWriter, ThresholdWriter};
