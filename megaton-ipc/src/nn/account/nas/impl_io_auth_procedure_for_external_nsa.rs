
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IOAuthProcedureForExternalNsa(Session);

impl AsRef<Session> for IOAuthProcedureForExternalNsa {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IOAuthProcedureForExternalNsa {
	pub fn prepare_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_request(&self, unk0: &mut ::nn::account::RequestUrl, unk1: &mut ::nn::account::CallbackUri) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn apply_response(&self, unk0: &[i8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn apply_response_async(&self, unk0: &[i8]) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn suspend(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_account_id(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_linked_nintendo_account_id(&self, ) -> Result<::nn::account::NintendoAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nickname(&self, unk0: &mut [i8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IOAuthProcedureForExternalNsa {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedureForExternalNsa {
		IOAuthProcedureForExternalNsa(Session::from_kobject(obj))
	}
}
