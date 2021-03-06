
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IOAuthProcedureForNintendoAccountLinkage<T>(T);

impl IOAuthProcedureForNintendoAccountLinkage<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IOAuthProcedureForNintendoAccountLinkage<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IOAuthProcedureForNintendoAccountLinkage(domain)),
			Err((sess, err)) => Err((IOAuthProcedureForNintendoAccountLinkage(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IOAuthProcedureForNintendoAccountLinkage<Session>> {
		Ok(IOAuthProcedureForNintendoAccountLinkage(self.0.duplicate()?))
	}
}

impl<T> Deref for IOAuthProcedureForNintendoAccountLinkage<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IOAuthProcedureForNintendoAccountLinkage<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IOAuthProcedureForNintendoAccountLinkage<T> {
	pub fn prepare_async(&self, ) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_request(&self, unk0: &mut ::ipcdefs::nn::account::RequestUrl, unk1: &mut ::ipcdefs::nn::account::CallbackUri) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn apply_response(&self, unk0: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn apply_response_async(&self, unk0: &[i8]) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn suspend(&self, ) -> Result<::ipcdefs::nn::account::detail::Uuid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let res : Response<::ipcdefs::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_request_with_theme(&self, unk0: i32, unk1: &mut ::ipcdefs::nn::account::RequestUrl, unk2: &mut ::ipcdefs::nn::account::CallbackUri) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(100)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_network_service_account_replaced(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_url_for_introduction_of_extra_membership(&self, unk0: &mut ::ipcdefs::nn::account::RequestUrl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(199)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IOAuthProcedureForNintendoAccountLinkage<T> {
	fn from(obj: T) -> IOAuthProcedureForNintendoAccountLinkage<T> {
		IOAuthProcedureForNintendoAccountLinkage(obj)
	}
}
