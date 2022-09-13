use crate::{Config, Creator, CreatorId, CreatorIdsForAccount, Creators, Error, Pallet};
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	/// Create new creator account with given id and add to account.
	pub fn add_new_creator_to_account(
		creator_id: CreatorId,
		account: T::AccountId,
	) -> Result<(), Error<T>> {
		// verify creator account does not exist
		ensure!(Self::creators(creator_id).is_none(), Error::<T>::CreatorAccountTaken);

		// add creator id to account
		CreatorIdsForAccount::<T>::mutate(account.clone(), |creator_ids| {
			// return error if unable to append creator account
			creator_ids
				.try_push(creator_id)
				.map_err(|_| Error::<T>::MaxCreatorAccountsReached)
		})?;

		// connect and save creator account
		Creators::<T>::insert(creator_id, Creator::new(creator_id, account));

		Ok(())
	}

	/// Remove creator account with given id from account.
	///
	/// Remove permanently if there are no token references to it.
	pub fn remove_creator_from_account(
		creator_id: CreatorId,
		account: T::AccountId,
	) -> Result<(), Error<T>> {
		let mut creator_ids = Self::creator_ids_for_account(account.clone());

		// verify account owns creator account
		if let Some(index) = creator_ids.iter().position(|id| *id == creator_id) {
			// disconnect and save creator account
			Creators::<T>::mutate(creator_id, |creator| {
				// this is ok because we are sure to only add creator ids of creator accounts that have been saved
				creator
					.as_mut()
					.expect("Creator removed without clearing references")
					.disconnect();
			});

			// remove creator id from account
			// `swap_remove` because we do not care about ordering and it is faster than `remove`
			creator_ids.swap_remove(index);
			// update storage
			CreatorIdsForAccount::<T>::insert(account, creator_ids);

			// remove if no token references to this creator
			// TODO! check tokens created by this creator and remove creator if none

			Ok(())
		} else {
			Err(Error::<T>::NotOwner)?
		}
	}
}
