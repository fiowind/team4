#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs


use frame_support::{decl_module, decl_storage, decl_event, decl_error,dispatch,ensure,
		traits::{Currency, Get, ExistenceRequirement},
		weights::{Weight, DispatchClass, FunctionOf, Pays},
	};
use frame_system::{self as system,ensure_signed};

use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;
// use pallet_balances::traits::Balance;


// type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

type BalanceOf<T> = <T as pallet_balances::Trait>::Balance;



// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// type Currency: Currency<Self::AccountId>;

	type MinClaimLength : Get<u32>;

	type MaxClaimLength : Get<u32>;
}



// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		// Something get(fn something): Option<u32>;

		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber,BalanceOf<T>);


	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where 
		AccountId = <T as system::Trait>::AccountId,
		Price = BalanceOf<T>,
	{
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		// SomethingStored(u32, AccountId),

		ClaimCreated(AccountId,Vec<u8>,Price),
		ClaimRemoved(AccountId,Vec<u8>),
		ClaimChanged(AccountId,Vec<u8>,AccountId,Price),
		ClaimPriceUpdated(AccountId,Vec<u8>,Price,Price),
		ClaimBuySuccess(AccountId,Vec<u8>,Price),

	}
);
// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		ProofAlreadyExist,
		ProofNotExist,
		NotHavePermission,
		CliamTooShort,
		CliamTooLong,
		ProofPriceIsSame,
		YourOwnProofs,
		PriceTooLow,
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

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		///  创建存证
		#[weight = 10_000]
		pub fn create_claim(origin, claim: Vec<u8>,price: BalanceOf<T>) -> dispatch::DispatchResult {



			ensure!(claim.len() as u32 >= T::MinClaimLength::get(),Error::<T>::CliamTooShort );

			ensure!(claim.len() as u32 <= T::MaxClaimLength::get(),Error::<T>::CliamTooLong );


			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist );

			Proofs::<T>::insert(&claim,(who.clone(),system::Module::<T>::block_number(),price.clone()));

			Self::deposit_event(RawEvent::ClaimCreated(who,claim,price));

			Ok(())

		}

		/// Another dummy entry point.
		/// takes no parameters, attempts to increment storage value, and possibly throws an error
		// #[weight = 10_000]
		// pub fn cause_error(origin) -> dispatch::DispatchResult {
		// 	// Check it was signed and get the signer. See also: ensure_root and ensure_none
		// 	let _who = ensure_signed(origin)?;

		// 	match Something::get() {
		// 		None => Err(Error::<T>::NoneValue)?,
		// 		Some(old) => {
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			Something::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }


		///  删除存证
		#[weight = 10_000]
		pub fn remove_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ProofNotExist );

			let (owner,_block_number,_) = Proofs::<T>::get(&claim);

			ensure!(owner == who, Error::<T>::NotHavePermission);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRemoved(who,claim));

			Ok(())

		}




		///  转让存证
		#[weight = 10_000]
		pub fn transfer_claim(origin, claim: Vec<u8>,receiver: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none

			//确认用户签名
			let who = ensure_signed(origin)?;


			//检查存证是否存在
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ProofNotExist );

			//检查存证的所有人
			let (owner,_block_number,price) = Proofs::<T>::get(&claim);


			//检查存证所有人与交易提交人是否一致
			ensure!(owner == who, Error::<T>::NotHavePermission);


			//查找接收人AccountId
			let receiver = T::Lookup::lookup(receiver)?;


			//修改存证所有人
			Proofs::<T>::mutate(&claim, |(_,__,price)| (receiver.clone(), system::Module::<T>::block_number(),price.clone()));


			//触发通知事件
			Self::deposit_event(RawEvent::ClaimChanged(who,claim,receiver,price));

			//返回
			Ok(())

		}



		///  修改存证价格
		#[weight = 10_000]
		pub fn update_claim_price(origin, claim: Vec<u8>,price: BalanceOf<T>) -> dispatch::DispatchResult {

			//确认用户签名
			let who = ensure_signed(origin)?;


			//检查存证是否存在
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ProofNotExist );

			//检查存证的所有人
			let (owner,_,old_price) = Proofs::<T>::get(&claim);


			//检查价格跟原价格是否一致
			ensure!(old_price != price,Error::<T>::ProofPriceIsSame );


			//检查存证所有人与交易提交人是否一致
			ensure!(owner == who, Error::<T>::NotHavePermission);


			//修改存证所有人
			Proofs::<T>::mutate(&claim, |(owner,_,price)| (owner.clone(), system::Module::<T>::block_number(),price.clone()));


			//触发通知事件
			Self::deposit_event(RawEvent::ClaimPriceUpdated(who,claim,old_price,price));

			//返回
			Ok(())

		}


		///  购买存证
		#[weight = FunctionOf(
			|(claim, price): (&Vec<u8>, &BalanceOf<T>)| (claim.len() * 10 + price / 100  + 10_000) as Weight,
			DispatchClass::Normal,
			Pays::Yes,
		)]
		pub fn buy_claim(origin, claim: Vec<u8>,price: BalanceOf<T>) -> dispatch::DispatchResult {

			//确认用户签名
			let who = ensure_signed(origin)?;


			//检查存证是否存在
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ProofNotExist );

			//检查存证的所有人
			let (owner,_,old_price) = Proofs::<T>::get(&claim);


			//检查购买人与所有人是否一致
			ensure!(owner != who,Error::<T>::YourOwnProofs );


			//检查新价格是否大于原价格
			ensure!(price >= old_price, Error::<T>::PriceTooLow);

			//将购买人支付的费用转到原所有人账户上
			T::Currency::transfer(&who, &owner, price, ExistenceRequirement::KeepAlive)?;


			//修改存证所有人
			Proofs::<T>::mutate(&claim, |(_,_,price)| (who.clone(), system::Module::<T>::block_number(),price.clone()));


			//触发通知事件
			Self::deposit_event(RawEvent::ClaimBuySuccess(who,claim,price));

			//返回
			Ok(())

		}




	}
}
