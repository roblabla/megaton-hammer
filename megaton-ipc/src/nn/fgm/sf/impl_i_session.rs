
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISession(Session);

impl ISession {
	pub fn new() -> Result<Arc<ISession>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISession>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"fgm:0\0\0\0").map(|s| Arc::new(unsafe { ISession::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		let r = sm.get_service(*b"fgm\0\0\0\0\0").map(|s| Arc::new(unsafe { ISession::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		let r = sm.get_service(*b"fgm:9\0\0\0").map(|s| Arc::new(unsafe { ISession::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISession {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISession {
	pub fn initialize(&self, ) -> Result<::nn::fgm::sf::IRequest> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ISession {
	unsafe fn from_kobject(obj: KObject) -> ISession {
		ISession(Session::from_kobject(obj))
	}
}
