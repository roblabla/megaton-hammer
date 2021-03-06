
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDeliveryCacheDirectoryService<T>(T);

impl IDeliveryCacheDirectoryService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDeliveryCacheDirectoryService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDeliveryCacheDirectoryService(domain)),
			Err((sess, err)) => Err((IDeliveryCacheDirectoryService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDeliveryCacheDirectoryService<Session>> {
		Ok(IDeliveryCacheDirectoryService(self.0.duplicate()?))
	}
}

impl<T> Deref for IDeliveryCacheDirectoryService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDeliveryCacheDirectoryService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDeliveryCacheDirectoryService<T> {
	pub fn open(&self, unk0: ::ipcdefs::nn::bcat::DirectoryName) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn read(&self, unk1: &mut [::ipcdefs::nn::bcat::DeliveryCacheDirectoryEntry]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_count(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IDeliveryCacheDirectoryService<T> {
	fn from(obj: T) -> IDeliveryCacheDirectoryService<T> {
		IDeliveryCacheDirectoryService(obj)
	}
}
