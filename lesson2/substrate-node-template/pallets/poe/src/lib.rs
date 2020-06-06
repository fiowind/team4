#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use sp_std::vec::Vec;
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_system::{self as system, ensure_signed};

const CLAIM_LENGTH: usize = 12;

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
	// ---------------------------------vvvvvvvvvvvvvv
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
		OutOfProofRange
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
		#[weight = 1_000]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {

			// verify the claim length not greater than CLAIM_LENGTH
			ensure!(claim.len() < CLAIM_LENGTH, Error::<T>::OutOfProofRange);

			// verify origin of proof is caller
			let sender = ensure_signed(origin)?;

			// verify the specified proof has not been claimed yet, or error that has already exist
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// Call the `system` pallet to get the current block number
			let block_number = system::Module::<T>::block_number();
			// Store the proof to the storage
			Proofs::<T>::insert(&claim, (sender.clone(), block_number));

			// Emit ClaimCreated event
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		/// Allow a owner to revoke their claim of created.
		#[weight = 1_000]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			// Verify caller
			let sender = ensure_signed(origin)?;

			// Verify the proof is exist, or error claim not found
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotFound);

			// get the owner
			let (owner, _) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

		#[weight = 1_000]
		pub fn transfer_claim(origin, claim: Vec<u8>, receiver: T::AccountId) -> dispatch::DispatchResult {
			// Verify caller
			let sender = ensure_signed(origin)?;

			// Verify the proof is exist, or error claim not found
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotFound);

			// get the owner
			let (owner, _) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			// Call the `system` pallet to get the current block number
			let block_number = system::Module::<T>::block_number();
			// Store the proof to the storage
			Proofs::<T>::insert(&claim, (receiver.clone(), block_number));

			Self::deposit_event(RawEvent::ClaimTransfered(sender, claim, receiver));

			Ok(())
		}
	}
}
