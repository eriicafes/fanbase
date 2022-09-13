use crate::Config;
use frame_support::pallet_prelude::*;

use super::{aliases::BalanceOf, CreatorId, MetatataUrl, MimeType, TokenId, TokenName};

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct LaunchToken<T: Config> {
	pub id: TokenId,
	pub creator: CreatorId,
	pub name: TokenName,
	pub price: BalanceOf<T>,
	pub mime_type: MimeType,
	pub metadata_url: MetatataUrl,
	// launch token specific fields
	pub supply: u64,
	pub last_mint: u64,
}
