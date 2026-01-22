//! Core business logic for battery management
//!
//! Provides battery information reading, vendor detection, threshold
//! management, power supply monitoring, peripheral device detection,
//! internationalization, and debug logging.

pub mod battery;
pub mod debug;
pub mod i18n;
pub mod peripheral;
pub mod power_supply;
#[cfg(test)]
pub mod traits;
pub mod vendor_detection;

pub use battery::BatteryInfo;
pub use peripheral::PeripheralBattery;
pub use power_supply::PowerSupplyInfo;
pub use vendor_detection::VendorInfo;
