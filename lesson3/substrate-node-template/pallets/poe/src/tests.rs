// Tests to be written here
//  shell command: cargo test -p pallet-poe


use super::*;
use crate::{Error,mock::*};

use frame_support::{assert_ok,assert_noop};

//teset cases for create_claim
#[test]
fn create_claim_failed_when_claim_too_short(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![1];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone(),1),
			Error::<Test>::CliamTooShort
			);
	})
}


#[test]
fn create_claim_failed_when_claim_too_long(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3,4,5,6,7,8,9];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone(),1),
			Error::<Test>::CliamTooLong
			);
	})
}


#[test]
fn create_claim_works(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone(),1));
		assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number(),1));
	})
}


#[test]  
fn create_claim_failed_when_proof_already_exist(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),1);

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone(),1),
			Error::<Test>::ProofAlreadyExist
			);
	})
}


//teset cases for remove_claim
#[test]  
fn remove_claim_failed_when_proof_not_exist(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];
		assert_noop!(
			PoeModule::remove_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::ProofNotExist
			);
	})
}


#[test]
fn remove_claim_failed_when_not_have_permission(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),1);

		assert_noop!(
			PoeModule::remove_claim(Origin::signed(2),claim.clone()),
			Error::<Test>::NotHavePermission
			);
	})
}

#[test]
fn remove_claim_works(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),1);


		assert_ok!(PoeModule::remove_claim(Origin::signed(1),claim.clone()));


		assert_ne!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number(),1),"Testing claim is removed.");


	})
}



//teset cases for transfer_claim
#[test]  
fn transfer_claim_failed_when_proof_not_exist(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2),
			Error::<Test>::ProofNotExist
			);
	})
}


#[test]
fn transfer_claim_failed_when_not_have_permission(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),1);

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2),claim.clone(),1),
			Error::<Test>::NotHavePermission
			);
	})
}

#[test]
fn transfer_claim_works(){
	ExtBuilder::build().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),1);

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2));

		assert_eq!(Proofs::<Test>::get(&claim),(2,system::Module::<Test>::block_number(),1));

	})
}


//teset cases for buy_claim
#[test]  
fn buy_claim_failed_when_your_own_proofs(){
	ExtBuilder::build().execute_with(||{

		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),1);

		assert_noop!(
			PoeModule::buy_claim(Origin::signed(1),claim.clone(),1),
			Error::<Test>::YourOwnProofs
			);
	})
}


#[test]  
fn buy_claim_failed_when_price_too_low(){
	ExtBuilder::build().execute_with(||{

		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),2);

		assert_noop!(
			PoeModule::buy_claim(Origin::signed(2),claim.clone(),1),
			Error::<Test>::PriceTooLow
			);
	})
}


#[test]  
fn buy_claim_works(){
	ExtBuilder::build().execute_with(||{

		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone(),1);

		assert_ok!(PoeModule::buy_claim(Origin::signed(2),claim.clone(),2));

		assert_eq!(Proofs::<Test>::get(&claim),(2,system::Module::<Test>::block_number(),2));

	})
}