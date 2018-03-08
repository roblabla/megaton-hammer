mod impl_IFactorySettingsServer;
pub use self::impl_IFactorySettingsServer::*;
mod impl_ISettingsServer;
pub use self::impl_ISettingsServer::*;
mod impl_IFirmwareDebugSettingsServer;
pub use self::impl_IFirmwareDebugSettingsServer::*;
mod impl_ISettingsItemKeyIterator;
pub use self::impl_ISettingsItemKeyIterator::*;
mod impl_ISystemSettingsServer;
pub use self::impl_ISystemSettingsServer::*;
pub mod system;
pub mod factory;
pub type SettingsName = ();
pub type LanguageCode = u64;
pub type SettingsItemKey = ();
