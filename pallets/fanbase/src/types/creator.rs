use crate::Config;
use frame_support::pallet_prelude::*;

/// CreatorId will represent a domain name element hence is restricted to max 63 bytes
pub type CreatorId = [u8; 63];

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Creator<T: Config> {
	pub id: CreatorId,
	pub owner: Option<T::AccountId>,
}
