
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IMonitorServiceCreator(Session);

impl IMonitorServiceCreator {
	pub fn GetMonitorService(&self, ) -> Result<(Session)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IMonitorServiceCreator {
	unsafe fn from_kobject(obj: KObject) -> IMonitorServiceCreator {
		IMonitorServiceCreator(Session::from_kobject(obj))
	}
}
