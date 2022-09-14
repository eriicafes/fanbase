use crate::Config;
use frame_support::pallet_prelude::*;

use super::{aliases::BalanceOf, CreatorId, MetatataUri, MimeType, TokenId, TokenName};

pub type TokenSupply = u32;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct LaunchToken<T: Config> {
	pub id: TokenId,
	pub creator: CreatorId,
	pub name: TokenName,
	pub price: BalanceOf<T>,
	pub mime_type: MimeType,
	pub metadata_uri: MetatataUri,
	// launch token specific fields
	pub supply: TokenSupply,
	pub issued: TokenSupply,
	pub destroyed: TokenSupply,
}

impl<T: Config> LaunchToken<T> {
	pub fn new(
		id: TokenId,
		creator: CreatorId,
		price: BalanceOf<T>,
		metadata: LaunchTokenMetadata,
	) -> Self {
		Self {
			id,
			creator,
			price,
			name: metadata.name,
			mime_type: metadata.mime_type,
			metadata_uri: metadata.metadata_uri,
			supply: metadata.supply,
			issued: 0,
			destroyed: 0,
		}
	}

	/// Increase issued count by 1.
	pub fn total_supply(&self) -> TokenSupply {
		self.supply.saturating_add(self.destroyed)
	}

	/// Increase issued count by 1.
	pub fn bump_issued(&mut self) {
		self.issued = self.issued.saturating_add(1);
	}

	/// Increase destroyed count by 1 and decrease supply count by 1.
	pub fn bump_destroyed_and_decrease_supply(&mut self) {
		self.supply = self.supply.saturating_sub(1);
		self.destroyed = self.destroyed.saturating_add(1);
	}
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LaunchTokenMetadata {
	pub name: TokenName,
	pub mime_type: MimeType,
	pub metadata_uri: MetatataUri,
	pub supply: TokenSupply,
}
