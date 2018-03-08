
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IDebugManager(Session);

impl IDebugManager {
	pub fn Unknown0(&self, ) -> Result<(Session)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IDebugManager {
	unsafe fn from_kobject(obj: KObject) -> IDebugManager {
		IDebugManager(Session::from_kobject(obj))
	}
}
