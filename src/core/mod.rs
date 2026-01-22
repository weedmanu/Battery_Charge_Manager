// Core module - Logique métier
pub mod battery;
pub mod power_supply;
pub mod vendor_detection;
pub mod traits;

pub use battery::BatteryInfo;
pub use power_supply::PowerSupplyInfo;
pub use vendor_detection::VendorInfo;
// Traits exportés pour API publique (utilisation future: DI, mocks, extensions)
#[allow(unused_imports)]
pub use traits::{BatteryService, SystemBatteryService, ThresholdWriter, SystemThresholdWriter};
