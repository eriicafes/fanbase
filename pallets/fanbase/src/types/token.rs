use crate::Config;
use frame_support::pallet_prelude::*;

use super::{aliases::BalanceOf, CreatorId, LaunchToken};

pub type TokenId = u128;

/// Token name limited to 255 bytes
pub type TokenName = BoundedVec<u8, ConstU32<255>>;

/// Token mime-type limited to 255 bytes
pub type MimeType = BoundedVec<u8, ConstU32<255>>;

/// Token metadata URI limited to 2048 bytes
pub type MetatataUri = BoundedVec<u8, ConstU32<2048>>;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Token<T: Config> {
	pub id: TokenId,
	pub launch_id: TokenId,
	pub creator: CreatorId,
	pub owner: T::AccountId,
	pub name: TokenName,
	pub price: Option<BalanceOf<T>>,
	pub mime_type: MimeType,
	pub metadata_uri: MetatataUri,
}

impl<T: Config> Token<T> {
	pub fn new(owner: T::AccountId, id: TokenId, launch_token: LaunchToken<T>) -> Self {
		Self {
			id,
			owner,
			launch_id: launch_token.id,
			creator: launch_token.creator,
			name: launch_token.name,
			price: None, // reset token price
			mime_type: launch_token.mime_type,
			metadata_uri: launch_token.metadata_uri,
		}
	}
}
