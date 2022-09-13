use crate::Config;
use frame_support::pallet_prelude::*;

use super::{aliases::BalanceOf, CreatorId, LaunchToken};

pub type TokenId = u128;

/// Token name limited to 255 bytes
pub type TokenName = [u8; 255];

/// Token mime-type limited to 255 bytes
pub type MimeType = [u8; 255];

/// Token metadata url limited to 2048 bytes
pub type MetatataUrl = [u8; 2048];

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Token<T: Config> {
	pub id: TokenId,
	pub creator: CreatorId,
	pub name: TokenName,
	pub price: Option<BalanceOf<T>>,
	pub mime_type: MimeType,
	pub metadata_uri: MetatataUrl,
}

impl<T: Config> From<LaunchToken<T>> for Token<T> {
	fn from(launch_token: LaunchToken<T>) -> Self {
		Self {
			id: launch_token.id,
			creator: launch_token.creator,
			name: launch_token.name,
			price: None, // reset token price
			mime_type: launch_token.mime_type,
			metadata_uri: launch_token.metadata_url,
		}
	}
}
