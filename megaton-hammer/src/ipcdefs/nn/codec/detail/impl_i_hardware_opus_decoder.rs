
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IHardwareOpusDecoder<T>(T);

impl IHardwareOpusDecoder<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IHardwareOpusDecoder<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHardwareOpusDecoder(domain)),
			Err((sess, err)) => Err((IHardwareOpusDecoder(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHardwareOpusDecoder<Session>> {
		Ok(IHardwareOpusDecoder(self.0.duplicate()?))
	}
}

impl<T> Deref for IHardwareOpusDecoder<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHardwareOpusDecoder<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHardwareOpusDecoder<T> {
	// fn decode_interleaved(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_context(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IHardwareOpusDecoder<T> {
	fn from(obj: T) -> IHardwareOpusDecoder<T> {
		IHardwareOpusDecoder(obj)
	}
}
