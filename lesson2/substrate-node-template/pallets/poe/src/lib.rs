#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	ensure, decl_module, decl_storage, decl_event, decl_error,
	dispatch, traits::{Get}};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	/// The minimum length a claim may be.
	type MinLength: Get<usize>;
	/// The maximum length a claim may be.
	type MaxLength: Get<usize>;
}

// /*
// {
//   "ClaimOwner": {
//     "acc": "[u8;32]",
//     "bn": "u32"
//   }
// }
//  */
// #[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq)]
// pub struct ClaimOwner<T> where T: Trait {
// 	acc: T::AccountId,
// 	bn: T::BlockNumber,
// }

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Poe {
		Proofs get(fn get_proof): map hasher(blake2_128_concat) Vec<u8> => Option<(T::AccountId, T::BlockNumber)>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
		BlockNumber = <T as system::Trait>::BlockNumber,
	{
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>, BlockNumber),
		/// ClaimTransferred: from, to, claim, block_number
		ClaimTransferred(AccountId, AccountId, Vec<u8>, BlockNumber),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ProofNotExist,
		NotClaimOwner,
		/// A claim is too short.
		TooShort,
		/// A claim is too long.
		TooLong,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		/// The minimum length a claim may be.
		const MinLength: u32 = T::MinLength::get() as u32;

		/// The maximum length a claim may be.
		const MaxLength: u32 = T::MaxLength::get() as u32;

		#[weight = 10_000]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(claim.len() >= T::MinLength::get(), Error::<T>::TooShort);
			ensure!(claim.len() <= T::MaxLength::get(), Error::<T>::TooLong);

			let o = Self::get_proof(&claim);
			ensure!(None == o, Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}

		#[weight = 10_000]
		pub fn revoke_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let (_acc, bn) = Self::must_get_with_owner(&sender, &claim)?;
			Proofs::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim, bn));
			Ok(())
		}

		#[weight = 10_000]
		pub fn transfer_claim(origin, claim:Vec<u8>, receiver: T::AccountId) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let (_acc, bn) = Self::must_get_with_owner(&sender, &claim)?;
			Proofs::<T>::insert(&claim, (receiver.clone(), bn));
			Self::deposit_event(RawEvent::ClaimTransferred(sender, receiver, claim, bn));
			Ok(())
		}
	}
}

impl<T> Module<T> where T: Trait {
	pub(crate) fn must_get_with_owner(sender: &T::AccountId, claim: &Vec<u8>) -> Result<(T::AccountId, T::BlockNumber), dispatch::DispatchError> {
		let o = Self::get_proof(&claim);
		ensure!(None != o, Error::<T>::ProofNotExist);
		let (acc, bn) = o.expect("must be a Some ;qed");
		ensure!(&acc == sender, Error::<T>::NotClaimOwner);
		Ok((acc, bn))
	}
}
