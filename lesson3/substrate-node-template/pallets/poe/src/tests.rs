// Tests to be written here
//  shell command: cargo test -p pallet-poe


use super::*;
use crate::{Error,mock::*};

use frame_support::{assert_ok,assert_noop};

//teset cases for create_claim
#[test]
fn create_claim_failed_when_claim_too_short(){
	new_test_ext().execute_with(||{
		let claim = vec![1];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::CliamTooShort
			);
	})
}


#[test]
fn create_claim_failed_when_claim_too_long(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,2,3,4,5,6,7,8,9];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::CliamTooLong
			);
	})
}


#[test]
fn create_claim_work(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,2,3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
	})
}


#[test]  
fn create_claim_failed_when_proof_already_exist(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::ProofAlreadyExist
			);
	})
}


//teset cases for remove_claim
#[test]  
fn remove_claim_failed_when_proof_not_exist(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,2,3];
		assert_noop!(
			PoeModule::remove_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::ProofNotExist
			);
	})
}


#[test]
fn remove_claim_failed_when_not_have_permission(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_noop!(
			PoeModule::remove_claim(Origin::signed(2),claim.clone()),
			Error::<Test>::NotHavePermission
			);
	})
}

#[test]
fn remove_claim_work(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,2,3];

		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());


		assert_ok!(PoeModule::remove_claim(Origin::signed(1),claim.clone()));


		assert_ne!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()),"Testing claim is removed.");


	})
}




