
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IContext<T>(T);

impl IContext<Session> {
	pub fn raw_new() -> Result<IContext<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"erpt:c\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IContext<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IContext<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"erpt:c\0\0") {
			let ret = Arc::new(IContext(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IContext<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IContext(domain)),
			Err((sess, err)) => Err((IContext(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IContext<Session>> {
		Ok(IContext(self.0.duplicate()?))
	}
}

impl<T> Deref for IContext<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IContext<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IContext<T> {
	// fn submit_context(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_report(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn unknown3(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn unknown4(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn unknown5(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IContext<T> {
	fn from(obj: T) -> IContext<T> {
		IContext(obj)
	}
}
