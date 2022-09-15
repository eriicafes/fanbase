use crate::{
	BalanceOf, Config, CreatorId, Error, IssuanceNonce, LaunchIssuanceNonce, LaunchToken,
	LaunchTokenIdsForCreator, LaunchTokenMetadata, LaunchTokens, Pallet, Token, TokenId,
	TokenIdsForAccount, Tokens,
};
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	/// Mint new launch token with provided price and metadata for creator.
	///
	/// Returns created launch token id.
	///
	/// *Unchecked!*
	///
	/// **Storage ops**
	/// - One storage read to get launch token issuance `LaunchIssuanceNonce<T>`
	/// - One storage read-write to add launch token id to creator `LaunchTokenIdsForCreator<T>`
	/// - One storage write to save launch token `LaunchTokens<T>`
	/// - One storage write to update launch token issuance `LaunchIssuanceNonce<T>`
	pub fn unchecked_mint(
		creator_id: CreatorId,
		price: BalanceOf<T>,
		metadata: LaunchTokenMetadata,
	) -> Result<TokenId, Error<T>> {
		// generate next launch token id
		let next_token_id = Self::launch_issuance_nonce()
			.checked_add(1)
			.ok_or(Error::<T>::LaunchTokensOverflow)?;

		// add launch token id to creator
		LaunchTokenIdsForCreator::<T>::try_mutate(&creator_id, |launch_token_ids| {
			launch_token_ids
				.try_push(next_token_id)
				.map_err(|_| Error::<T>::MaxLaunchTokensReached)
		})?;

		// save launch token
		LaunchTokens::<T>::insert(
			&next_token_id,
			LaunchToken::new(next_token_id, creator_id, price, metadata),
		);

		// update nonce
		LaunchIssuanceNonce::<T>::set(next_token_id);

		Ok(next_token_id)
	}

	/// Get token from launch token and transfer to account.
	///
	/// *Unchecked!*
	///
	/// **Storage ops**
	/// - One storage read to get token issuance `IssuanceNonce<T>`
	/// - One storage read to get launch token by id `LaunchTokens<T>`
	/// - One storage read-write to add token id to receiver account `TokenIdsForAccount<T>`
	/// - One storage write to save token `Tokens<T>`
	/// - One storage write to update launch token internal issuance `LaunchTokens<T>`
	/// - One storage write to update token issuance `IssuanceNonce<T>`
	pub fn unchecked_launch_transfer(
		receiver: &T::AccountId,
		launch_token_id: &TokenId,
	) -> Result<TokenId, Error<T>> {
		// generate next token id
		let next_token_id =
			Self::issuance_nonce().checked_add(1).ok_or(Error::<T>::TokensOverflow)?;

		// get launch token
		let launch_token = Self::launch_tokens(launch_token_id).ok_or(Error::<T>::TokenNotFound)?;

		// ensure issuance does not exceed total supply
		if launch_token.issued < launch_token.total_supply() {
			// add token id to account
			TokenIdsForAccount::<T>::try_mutate(receiver, |token_ids| {
				token_ids.try_push(next_token_id).map_err(|_| Error::<T>::MaxTokensReached)
			})?;

			// save token
			Tokens::<T>::insert(
				&next_token_id,
				Token::new(receiver.clone(), next_token_id, launch_token),
			);

			// update launch token
			LaunchTokens::<T>::mutate(launch_token_id, |launch_token| {
				// unwrap because we are sure launch_token exists
				launch_token.as_mut().unwrap().bump_issued();
			});

			// update nonce
			IssuanceNonce::<T>::set(next_token_id);

			Ok(next_token_id)
		} else {
			Err(Error::<T>::TokenSoldOut)
		}
	}

	/// Remove token from owner and transfer to receiver.
	///
	/// *Unchecked!*
	///
	/// **Storage ops**
	/// - One storage read to get token by id `Tokens<T>`
	/// - One storage read-write to add token id to receiver account `TokenIdsForAccount<T>`
	/// - One storage read-write to remove token id from owner account `TokenIdsForAccount<T>`
	/// - One storage write to update token owner `Tokens<T>`
	pub fn unchecked_transfer(
		owner: &T::AccountId,
		receiver: &T::AccountId,
		token_id: &TokenId,
	) -> Result<(), Error<T>> {
		Tokens::<T>::try_mutate(token_id, |token| {
			// check if token exists
			let token = token.as_mut().ok_or(Error::<T>::TokenNotFound)?;

			// add token id to receiver
			TokenIdsForAccount::<T>::try_mutate(receiver, |token_ids| {
				token_ids.try_push(token_id.clone()).map_err(|_| Error::<T>::MaxTokensReached)
			})?;

			// remove token id from owner
			TokenIdsForAccount::<T>::mutate(owner, |token_ids| {
				if let Some(index) = token_ids.iter().position(|id| id == token_id) {
					// `swap_remove` because we do not care about ordering and it is faster than `remove`
					token_ids.swap_remove(index);
				}
			});

			// update token owner
			token.owner = receiver.clone();

			Ok(())
		})
	}

	/// Set price for launch token.
	///
	/// *Unchecked!*
	///
	/// **Storage ops**
	/// - One storage read-write to update launch token price `LaunchTokens<T>`
	pub fn unchecked_set_launch_price(
		launch_token_id: &TokenId,
		price: BalanceOf<T>,
	) -> Result<(), Error<T>> {
		LaunchTokens::<T>::try_mutate(launch_token_id, |launch_token| {
			// check if launch token exists
			let launch_token = launch_token.as_mut().ok_or(Error::<T>::TokenNotFound)?;

			// update price
			launch_token.price = price;

			Ok(())
		})
	}

	/// Set price for token.
	///
	/// *Unchecked!*
	///
	/// **Storage ops**
	/// - One storage read-write to update token price `Tokens<T>`
	pub fn unchecked_set_price(
		token_id: &TokenId,
		price: Option<BalanceOf<T>>,
	) -> Result<(), Error<T>> {
		Tokens::<T>::try_mutate(token_id, |token| {
			// check if token exists
			let token = token.as_mut().ok_or(Error::<T>::TokenNotFound)?;

			// update price
			token.price = price;

			Ok(())
		})
	}

	/// Destroy token.
	///
	/// *Unchecked!*
	///
	/// **Storage ops**
	/// - One storage read to get token by id `Tokens<T>`
	/// - One storage read-write to remove token id from token owner account `TokenIdsForAccount<T>`
	/// - One storage write to remove token `Tokens<T>`
	/// - One storage read-write to update launch token internal issuance `LaunchTokens<T>`
	pub fn unchecked_burn(token_id: &TokenId) -> Result<(), Error<T>> {
		let token = Self::tokens(token_id).ok_or(Error::<T>::TokenNotFound)?;

		// remove token id from owner
		TokenIdsForAccount::<T>::mutate(&token.owner, |token_ids| {
			if let Some(index) = token_ids.iter().position(|id| *id == token.id) {
				// `swap_remove` because we do not care about ordering and it is faster than `remove`
				token_ids.swap_remove(index);
			}
		});

		// remove token
		Tokens::<T>::remove(&token.id);

		// update launch token
		LaunchTokens::<T>::mutate(&token.launch_id, |launch_token| {
			// unwrap because we are sure launch_token exists
			launch_token.as_mut().unwrap().bump_destroyed_and_decrease_supply();
		});

		Ok(())
	}

	/// Ensure creator account owns launch token.
	///
	/// **Storage ops**
	/// - One storage read to get launch token by id `LaunchTokens<T>`
	pub fn ensure_creator_owns_launch_token(
		creator_id: &CreatorId,
		launch_token_id: &TokenId,
	) -> Result<(), Error<T>> {
		ensure!(
			Self::launch_tokens(launch_token_id)
				.map_or(false, |launch_token| &launch_token.creator == creator_id),
			Error::<T>::NotOwner
		);

		Ok(())
	}

	/// Ensure account owns token.
	///
	/// **Storage ops**
	/// - One storage read to get token by id `Tokens<T>`
	pub fn ensure_account_owns_token(
		account: &T::AccountId,
		token_id: &TokenId,
	) -> Result<(), Error<T>> {
		ensure!(
			Self::tokens(token_id).map_or(false, |token| token.owner == *account),
			Error::<T>::NotOwner
		);

		Ok(())
	}

	/// Get launch token owner if launch token exists and it's creator's owner has not been disconnected.
	///
	/// **Storage ops**
	/// - One storage read to get launch token by id `LaunchTokens<T>`
	/// - One storage read to get creator of launch token `Creators<T>`
	pub fn get_launch_token_owner(launch_token_id: &TokenId) -> Option<T::AccountId> {
		Self::launch_tokens(launch_token_id).and_then(|launch_token| {
			Self::creators(launch_token.creator).and_then(|creator| creator.owner)
		})
	}

	/// Get token price if token exists and has a price.
	///
	/// **Storage ops**
	/// - One storage read to get token by id `Tokens<T>`
	pub fn get_token_price(token_id: &TokenId) -> Option<BalanceOf<T>> {
		Self::tokens(token_id).and_then(|token| token.price)
	}
}
