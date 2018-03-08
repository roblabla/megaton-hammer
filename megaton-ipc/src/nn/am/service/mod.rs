mod impl_IAppletAccessor;
pub use self::impl_IAppletAccessor::*;
mod impl_ILibraryAppletAccessor;
pub use self::impl_ILibraryAppletAccessor::*;
mod impl_ISelfController;
pub use self::impl_ISelfController::*;
mod impl_IWindowController;
pub use self::impl_IWindowController::*;
mod impl_IAllSystemAppletProxiesService;
pub use self::impl_IAllSystemAppletProxiesService::*;
mod impl_ILibraryAppletCreator;
pub use self::impl_ILibraryAppletCreator::*;
mod impl_IApplicationProxyService;
pub use self::impl_IApplicationProxyService::*;
mod impl_IApplicationFunctions;
pub use self::impl_IApplicationFunctions::*;
mod impl_IStorage;
pub use self::impl_IStorage::*;
mod impl_IApplicationProxy;
pub use self::impl_IApplicationProxy::*;
mod impl_ILockAccessor;
pub use self::impl_ILockAccessor::*;
mod impl_IGlobalStateController;
pub use self::impl_IGlobalStateController::*;
mod impl_ISystemAppletProxy;
pub use self::impl_ISystemAppletProxy::*;
mod impl_IOverlayFunctions;
pub use self::impl_IOverlayFunctions::*;
mod impl_IDebugFunctions;
pub use self::impl_IDebugFunctions::*;
mod impl_IOverlayAppletProxy;
pub use self::impl_IOverlayAppletProxy::*;
mod impl_IAudioController;
pub use self::impl_IAudioController::*;
mod impl_IWindow;
pub use self::impl_IWindow::*;
mod impl_ILibraryAppletProxy;
pub use self::impl_ILibraryAppletProxy::*;
mod impl_ITransferStorageAccessor;
pub use self::impl_ITransferStorageAccessor::*;
mod impl_IDisplayController;
pub use self::impl_IDisplayController::*;
mod impl_IHomeMenuFunctions;
pub use self::impl_IHomeMenuFunctions::*;
mod impl_ICommonStateGetter;
pub use self::impl_ICommonStateGetter::*;
mod impl_IApplicationAccessor;
pub use self::impl_IApplicationAccessor::*;
mod impl_IProcessWindingController;
pub use self::impl_IProcessWindingController::*;
mod impl_IStorageAccessor;
pub use self::impl_IStorageAccessor::*;
mod impl_ILibraryAppletSelfAccessor;
pub use self::impl_ILibraryAppletSelfAccessor::*;
mod impl_IApplicationCreator;
pub use self::impl_IApplicationCreator::*;
pub type AppletIdentityInfo = u128;
pub type AppletProcessLaunchReason = u32;
pub type LibraryAppletInfo = u64;
pub type WindowCreationOption = u32;
pub type AppletKind = u64;
pub type EmulatedButtonEvent = u32;
