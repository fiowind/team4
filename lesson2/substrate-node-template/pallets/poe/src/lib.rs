#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use sp_std::vec::Vec;
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{self as system, ensure_signed};

const RANGE: usize = 12;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Event emitted when a proof has been created.
		ClaimCreated(AccountId, Vec<u8>),
		/// Event emitted when a claim  is revoked by the owner(AccountId).
		ClaimRevoked(AccountId, Vec<u8>),
		/// Event emitted when a claim is transfer
		ClaimTransfered(AccountId, Vec<u8>, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// This proof has already been claimed
		ProofAlreadyExist,
		/// The proof doesn't exist
		ClaimNotFound,
		/// Can't handle claim when handler is not the owner
		NotClaimOwner,
		/// The proof too long after vec size greater than 12
		OutOfRange
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

		/// Allow a user to claim ownership of an unclaimed proof
		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {

			ensure!(claim.len() < RANGE, Error::<T>::OutOfRange);

			let sender = ensure_signed(origin)?;
			
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
			
			let block_number = system::Module::<T>::block_number();
			
			Proofs::<T>::insert(&claim, (sender.clone(), block_number));

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		/// Allow a owner to revoke their claim of created.
		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotFound);
			
			let (owner, _) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>, receiver: T::AccountId) -> dispatch::DispatchResult {
			
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotFound);

			let (owner, _) = Proofs::<T>::get(&claim);
			
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			let block_number = system::Module::<T>::block_number();

			Proofs::<T>::insert(&claim, (receiver.clone(), block_number));

			Self::deposit_event(RawEvent::ClaimTransfered(sender, claim, receiver));

			Ok(())
		}
	}
}