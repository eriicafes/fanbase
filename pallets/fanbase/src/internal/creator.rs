use crate::{Config, Creator, CreatorId, CreatorIdsForAccount, Creators, Error, Pallet};
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	/// Create new creator account with given id and add to account.
	///
	/// **Storage ops**
	/// - One storage read to get creator by id `Creators<T>`
	/// - One storage read-write to add creator id to account `CreatorIdsForAccount<T>`
	/// - One storage write to save creator `Creators<T>`
	pub fn add_new_creator_to_account(
		creator_id: CreatorId,
		account: T::AccountId,
	) -> Result<(), Error<T>> {
		// verify creator account does not exist
		ensure!(Self::creators(&creator_id).is_none(), Error::<T>::CreatorAccountTaken);

		// add creator id to account
		CreatorIdsForAccount::<T>::try_mutate(&account, |creator_ids| {
			// return error if unable to append creator account
			creator_ids
				.try_push(creator_id.clone())
				.map_err(|_| Error::<T>::MaxCreatorAccountsReached)
		})?;

		// connect and save creator account
		Creators::<T>::insert(&creator_id, Creator::new(creator_id.clone(), account));

		Ok(())
	}

	/// Remove creator account with given id from account.
	///
	/// Remove permanently if there are no token references to it.
	///
	/// **Storage ops**
	/// - One storage read to get creator by id `Creators<T>`
	/// - One storage read to get launch tokens ids for creator `LaunchTokenIdsForCreator<T>`
	/// - One storage write to either disconnect or remove creator `Creators<T>`
	/// - One storage read-write to remove creator id from account `CreatorIdsForAccount<T>`
	pub fn remove_creator_from_account(
		creator_id: CreatorId,
		account: T::AccountId,
	) -> Result<(), Error<T>> {
		// verify account owns creator account
		Self::ensure_account_owns_creator(&account, &creator_id)?;

		// remove if no token references to this creator
		if Self::launch_token_ids_for_creator(&creator_id).len() == 0 {
			// remove since no launch tokens created by this creator
			Creators::<T>::remove(&creator_id);
		} else {
			// disconnect owner from creator
			Creators::<T>::mutate(&creator_id, |creator| {
				// unwrap because we are sure creator exists
				creator.as_mut().unwrap().disconnect();
			})
		}

		// remove creator id from account
		CreatorIdsForAccount::<T>::mutate(&account, |creator_ids| {
			if let Some(index) = creator_ids.iter().position(|id| *id == creator_id) {
				// `swap_remove` because we do not care about ordering and it is faster than `remove`
				creator_ids.swap_remove(index);
			}
		});

		Ok(())
	}

	/// Ensure account owns creator account.
	///
	/// **Storage ops**
	/// - One storage read to get creator by id `Creators<T>`
	pub fn ensure_account_owns_creator(
		account: &T::AccountId,
		creator_id: &CreatorId,
	) -> Result<(), Error<T>> {
		ensure!(
			Self::creators(creator_id)
				.map_or(false, |creator| creator.owner == Some(account.clone())),
			Error::<T>::NotOwner
		);

		Ok(())
	}
}
