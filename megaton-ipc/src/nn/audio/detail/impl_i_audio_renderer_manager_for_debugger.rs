
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAudioRendererManagerForDebugger<T>(T);

impl IAudioRendererManagerForDebugger<Session> {
	pub fn new() -> Result<Arc<IAudioRendererManagerForDebugger<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAudioRendererManagerForDebugger<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"audren:d") {
			let ret = Arc::new(IAudioRendererManagerForDebugger(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"audren:d").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IAudioRendererManagerForDebugger<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAudioRendererManagerForDebugger(domain)),
			Err((sess, err)) => Err((IAudioRendererManagerForDebugger(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAudioRendererManagerForDebugger<Session>> {
		Ok(IAudioRendererManagerForDebugger(self.0.duplicate()?))
	}
}

impl<T> Deref for IAudioRendererManagerForDebugger<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAudioRendererManagerForDebugger<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAudioRendererManagerForDebugger<T> {
	pub fn unknown0(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAudioRendererManagerForDebugger<T> {
	fn from(obj: T) -> IAudioRendererManagerForDebugger<T> {
		IAudioRendererManagerForDebugger(obj)
	}
}
