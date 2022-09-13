#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;
mod weights;

use types::{Creator, CreatorId, LaunchToken, Token, TokenId};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// CONFIG
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Emit events.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Internal currency.
		type Currency: Currency<Self::AccountId>;

		/// Max creator accounts for account
		#[pallet::constant]
		type MaxCreatorAccounts: Get<u32>;

		/// Max launch tokens for creator
		#[pallet::constant]
		type MaxLaunchTokens: Get<u32>;

		/// Max tokens for account
		#[pallet::constant]
		type MaxTokens: Get<u32>;
	}

	// STORAGE ITEMS
	/// Creator accounts
	#[pallet::storage]
	#[pallet::getter(fn creators)]
	pub type Creators<T> = StorageMap<_, Blake2_128Concat, CreatorId, Creator<T>>;

	/// Creators for account
	#[pallet::storage]
	#[pallet::getter(fn creators_for_account)]
	pub type CreatorsForAccount<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<CreatorId, T::MaxCreatorAccounts>>;

	/// Launch tokens
	#[pallet::storage]
	#[pallet::getter(fn launch_tokens)]
	pub type LaunchTokens<T: Config> = StorageMap<_, Blake2_128Concat, TokenId, LaunchToken<T>>;

	/// Launch tokens for creator
	#[pallet::storage]
	#[pallet::getter(fn launch_tokens_for_creator)]
	pub type LaunchTokensForCreator<T: Config> =
		StorageMap<_, Blake2_128Concat, CreatorId, BoundedVec<TokenId, T::MaxLaunchTokens>>;

	/// Tokens
	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub type Tokens<T: Config> = StorageMap<_, Blake2_128Concat, TokenId, Token<T>>;

	/// Tokens for account
	#[pallet::storage]
	#[pallet::getter(fn tokens_for_creator)]
	pub type TokensForAccount<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<TokenId, T::MaxTokens>>;

	/// Token issuance
	#[pallet::storage]
	#[pallet::getter(fn issuance_nonce)]
	pub type IssuanceNonce<T> = StorageValue<_, TokenId>;

	// EVENTS
	#[pallet::event]
	pub enum Event<T: Config> {
		/// new creator account created
		NewCreator,

		/// creator account dropped
		DroppedCreator,

		/// new token minted
		TokenCreated,

		/// token acquired for the first time
		TokenInitialCollection,

		/// token transferred to new owner
		TokenTransferred,

		/// token listed on market
		TokenListed,

		/// token unlisted from market
		TokenUnlisted,

		/// token launch price updated
		TokenLaunchPriceUpdated,

		/// token price updated
		TokenPriceUpdated,

		/// token permanently destroyed
		TokenDestroyed,
	}

	// ERRORS
	#[pallet::error]
	pub enum Error<T> {
		/// Signing account is not the owner of this item
		NotOwner,

		/// Creator account already taken
		CreatorAccountTaken,

		/// Token sold out of launch
		TokenSoldOut,

		/// Token not for sale
		TokenNotForSale,

		/// Bid price too low to buy token
		BidPriceTooLow,

		/// Cannot set token supply to zero
		ZeroSupply,

		/// Cannot set token price to zero
		ZeroPrice,

		/// Cannot transfer token to self
		TransferToSelf,

		/// Max number of creator accounts reached
		MaxCreatorAccountsReached,

		/// Max number of launch tokens reached
		MaxLaunchTokensReached,

		/// Max number of tokens reached
		MaxTokensReached,

		/// Max tokens minted
		TokensOverflow,
	}

	// CALLS
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create new creator account.
		#[pallet::weight(weights::HIGH + T::DbWeight::get().reads_writes(1, 1))]
		pub fn create_account(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Drop creator account.
		///
		/// Keeps creator account alive if tokens have been created by the creator account.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn drop_account(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Create new token.
		#[pallet::weight(weights::HIGH + T::DbWeight::get().reads_writes(1,1))]
		pub fn mint(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Gift token to account first hand.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn launch_gift(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Buy token from creator first hand.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn launch_buy(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Buy token from market.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn buy(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Transfer token to account.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn transfer(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// List token on market.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn list(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Unlist token from market.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn unlist(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Update launch price of token.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn set_launch_price(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Update price of token.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn set_price(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}

		/// Destroy token.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(1,1))]
		pub fn burn(_origin: OriginFor<T>) -> DispatchResult {
			todo!()
		}
	}
}
