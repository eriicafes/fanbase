use crate::Config;
use frame_support::pallet_prelude::*;

/// CreatorId will represent a domain name element hence is restricted to max 63 bytes
pub type CreatorId = BoundedVec<u8, ConstU32<63>>;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Creator<T: Config> {
	pub id: CreatorId,
	pub owner: Option<T::AccountId>,
}

impl<T: Config> Creator<T> {
	pub fn new(id: CreatorId, owner: T::AccountId) -> Self {
		Self { id, owner: Some(owner) }
	}

	/// Remove owner from creator by setting owner field to `None`
	pub fn disconnect(&mut self) {
		self.owner = None
	}
}
