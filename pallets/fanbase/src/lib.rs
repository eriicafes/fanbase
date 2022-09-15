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

mod internal;
pub mod types;
mod weights;

use types::{
	aliases::BalanceOf, Creator, CreatorId, LaunchToken, LaunchTokenMetadata, Token, TokenId,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement::KeepAlive},
	};
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

	/// Creator ids for account.
	/// Maps Accounts to their creator accounts.
	#[pallet::storage]
	#[pallet::getter(fn creator_ids_for_account)]
	pub type CreatorIdsForAccount<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<CreatorId, T::MaxCreatorAccounts>,
		ValueQuery,
	>;

	/// Launch tokens for creators.
	#[pallet::storage]
	#[pallet::getter(fn launch_tokens)]
	pub type LaunchTokens<T: Config> = StorageMap<_, Blake2_128Concat, TokenId, LaunchToken<T>>;

	/// Launch token ids for creator.
	/// Maps creators to their launch tokens.
	#[pallet::storage]
	#[pallet::getter(fn launch_token_ids_for_creator)]
	pub type LaunchTokenIdsForCreator<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		CreatorId,
		BoundedVec<TokenId, T::MaxLaunchTokens>,
		ValueQuery,
	>;

	/// Tokens for accounts.
	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub type Tokens<T: Config> = StorageMap<_, Blake2_128Concat, TokenId, Token<T>>;

	/// Token ids for accounts.
	/// Maps accounts to their tokens.
	#[pallet::storage]
	#[pallet::getter(fn token_ids_for_account)]
	pub type TokenIdsForAccount<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<TokenId, T::MaxTokens>,
		ValueQuery,
	>;

	/// Track issued launch tokens count
	#[pallet::storage]
	#[pallet::getter(fn launch_issuance_nonce)]
	pub type LaunchIssuanceNonce<T> = StorageValue<_, TokenId, ValueQuery>;

	/// Track issued tokens count
	#[pallet::storage]
	#[pallet::getter(fn issuance_nonce)]
	pub type IssuanceNonce<T> = StorageValue<_, TokenId, ValueQuery>;

	// EVENTS
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New creator account created
		NewCreator,

		/// Creator account dropped
		DroppedCreator,

		/// New token minted
		TokenCreated,

		/// Token acquired for the first time
		TokenInitialCollection,

		/// Token transferred to new owner
		TokenTransferred,

		/// Token listed on market
		TokenListed,

		/// Token unlisted from market
		TokenUnlisted,

		/// Token launch price updated
		TokenLaunchPriceUpdated,

		/// Token price updated
		TokenPriceUpdated,

		/// Token permanently destroyed
		TokenDestroyed,
	}

	// ERRORS
	#[pallet::error]
	pub enum Error<T> {
		/// Insufficient funds to complete buy operation
		InsufficientFunds,

		/// Signing account is not the owner of this item
		NotOwner,

		/// Creator account already taken
		CreatorAccountTaken,

		/// Token not found
		TokenNotFound,

		/// Token sold out of launch
		TokenSoldOut,

		/// Token not for sale
		TokenNotForSale,

		/// Token creator is unavailable
		TokenUnavailable,

		/// Token not listed
		TokenNotListed,

		/// Token already listed
		TokenAlreadyListed,

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

		/// Max launch tokens minted
		LaunchTokensOverflow,

		/// Max tokens minted
		TokensOverflow,
	}

	// CALLS
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create new creator account.
		#[pallet::weight(weights::HIGH + T::DbWeight::get().reads_writes(2, 2))]
		pub fn create_account(origin: OriginFor<T>, creator_id: CreatorId) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			Self::add_new_creator_to_account(creator_id, account)?;

			// emit events
			Self::deposit_event(Event::<T>::NewCreator);

			Ok(())
		}

		/// Drop creator account.
		///
		/// Keeps creator account alive if tokens have been created by the creator account.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(3, 2))]
		pub fn drop_account(origin: OriginFor<T>, creator_id: CreatorId) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			Self::remove_creator_from_account(creator_id, account)?;

			// emit events
			Self::deposit_event(Event::<T>::DroppedCreator);

			Ok(())
		}

		/// Create new token.
		#[pallet::weight(weights::HIGH + T::DbWeight::get().reads_writes(3, 3))]
		pub fn mint(
			origin: OriginFor<T>,
			creator_id: CreatorId,
			price: BalanceOf<T>,
			metadata: LaunchTokenMetadata,
		) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// verify account owns creator account
			Self::ensure_account_owns_creator(&account, &creator_id)?;

			// mint launch token
			Self::unchecked_mint(creator_id, price, metadata)?;

			// emit events
			Self::deposit_event(Event::<T>::TokenCreated);

			Ok(())
		}

		/// Gift token to account first hand.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(4, 4))]
		pub fn launch_gift(
			origin: OriginFor<T>,
			creator_id: CreatorId,
			launch_token_id: TokenId,
			receiver: T::AccountId,
		) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// verify account owns creator account
			Self::ensure_account_owns_creator(&account, &creator_id)?;
			// verify creator account owns launch token
			Self::ensure_creator_owns_launch_token(&creator_id, &launch_token_id)?;

			// transfer token to receiver
			Self::unchecked_launch_transfer(&receiver, launch_token_id)?;

			// emit events
			Self::deposit_event(Event::<T>::TokenInitialCollection);

			Ok(())
		}

		/// Buy token from creator first hand.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(5, 4))]
		pub fn launch_buy(
			origin: OriginFor<T>,
			launch_token_id: TokenId,
			bid_price: BalanceOf<T>,
		) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// ensure sufficient balance
			ensure!(
				T::Currency::free_balance(&account) >= bid_price,
				Error::<T>::InsufficientFunds
			);

			let launch_token =
				Self::launch_tokens(launch_token_id).ok_or(Error::<T>::TokenNotFound)?;

			// get launch token owner
			let launch_token_owner = Self::get_launch_token_owner(&launch_token_id)
				.ok_or(Error::<T>::TokenUnavailable)?;

			// ensure bid price is enough to cover purchase
			ensure!(bid_price >= launch_token.price, Error::<T>::BidPriceTooLow);

			// transfer token to receiver from launch token
			Self::unchecked_launch_transfer(&account, launch_token_id)?;

			// transfer funds
			T::Currency::transfer(&account, &launch_token_owner, bid_price, KeepAlive)
				.expect("Funds not transferred after token transfer");

			// emit events
			Self::deposit_event(Event::<T>::TokenInitialCollection);

			Ok(())
		}

		/// Buy token from market.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(4, 3))]
		pub fn buy(
			origin: OriginFor<T>,
			token_id: TokenId,
			bid_price: BalanceOf<T>,
		) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// ensure sufficient balance
			ensure!(
				T::Currency::free_balance(&account) >= bid_price,
				Error::<T>::InsufficientFunds
			);

			let token = Self::tokens(token_id).ok_or(Error::<T>::TokenNotFound)?;

			// get if token price, return error if not for sale
			let token_price = token.price.ok_or(Error::<T>::TokenNotForSale)?;

			// ensure bid price is enough to cover purchase
			ensure!(bid_price >= token_price, Error::<T>::BidPriceTooLow);

			// transfer token from owner to account
			Self::unchecked_transfer(&token.owner, &account, token_id)?;

			// transfer funds
			T::Currency::transfer(&account, &token.owner, bid_price, KeepAlive)
				.expect("Funds not transferred after token transfer");

			// emit events
			Self::deposit_event(Event::<T>::TokenTransferred);

			Ok(())
		}

		/// Transfer token to account.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(3, 3))]
		pub fn transfer(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// check if token exists and return `NotFound` error early
			Self::tokens(token_id).ok_or(Error::<T>::TokenNotFound)?;

			// ensure account owns token
			Self::ensure_account_owns_token(&account, &token_id)?;

			// transfer token to receiver
			Self::unchecked_transfer(&account, &account, token_id)?;

			// emit events
			Self::deposit_event(Event::<T>::TokenTransferred);

			Ok(())
		}

		/// List token on market.
		#[pallet::weight(weights::LOW + T::DbWeight::get().reads_writes(1, 1))]
		pub fn list(
			origin: OriginFor<T>,
			token_id: TokenId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// ensure account owns token
			Self::ensure_account_owns_token(&account, &token_id)?;

			// ensure token does not have a price
			ensure!(Self::get_token_price(&token_id).is_none(), Error::<T>::TokenAlreadyListed);

			Self::unchecked_set_price(token_id, Some(price))?;

			// emit events
			Self::deposit_event(Event::<T>::TokenListed);

			Ok(())
		}

		/// Unlist token from market.
		#[pallet::weight(weights::LOW + T::DbWeight::get().reads_writes(1, 1))]
		pub fn unlist(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// ensure account owns token
			Self::ensure_account_owns_token(&account, &token_id)?;

			// ensure token has price
			ensure!(Self::get_token_price(&token_id).is_some(), Error::<T>::TokenNotListed);

			// update token price
			Self::unchecked_set_price(token_id, None)?;

			// emit events
			Self::deposit_event(Event::<T>::TokenUnlisted);

			Ok(())
		}

		/// Update launch price of token.
		#[pallet::weight(weights::LOW + T::DbWeight::get().reads_writes(2, 1))]
		pub fn set_launch_price(
			origin: OriginFor<T>,
			creator_id: CreatorId,
			launch_token_id: TokenId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// verify account owns creator account
			Self::ensure_account_owns_creator(&account, &creator_id)?;
			// verify creator account owns launch token
			Self::ensure_creator_owns_launch_token(&creator_id, &launch_token_id)?;

			// update launch token price
			Self::unchecked_set_launch_price(launch_token_id, price)?;

			// emit events
			Self::deposit_event(Event::<T>::TokenLaunchPriceUpdated);

			Ok(())
		}

		/// Update price of token.
		#[pallet::weight(weights::LOW + T::DbWeight::get().reads_writes(1, 1))]
		pub fn set_price(
			origin: OriginFor<T>,
			token_id: TokenId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// ensure account owns token
			Self::ensure_account_owns_token(&account, &token_id)?;

			// ensure token has price
			ensure!(Self::get_token_price(&token_id).is_some(), Error::<T>::TokenNotListed);

			// update token price
			Self::unchecked_set_price(token_id, Some(price))?;

			// emit events
			Self::deposit_event(Event::<T>::TokenPriceUpdated);

			Ok(())
		}

		/// Destroy token.
		#[pallet::weight(weights::MID + T::DbWeight::get().reads_writes(3, 3))]
		pub fn burn(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
			// allow only signed origin
			let account = ensure_signed(origin)?;

			// ensure account owns token
			Self::ensure_account_owns_token(&account, &token_id)?;

			Self::unchecked_burn(token_id)?;

			// emit events
			Self::deposit_event(Event::<T>::TokenDestroyed);

			Ok(())
		}
	}
}
