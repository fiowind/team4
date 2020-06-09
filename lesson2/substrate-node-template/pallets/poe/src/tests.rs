// Tests to be written here

use crate::{Error, Proofs, mock::*};
use frame_support::{assert_ok, assert_noop, StorageMap};

#[test]
fn it_lets_user_create_claims() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2,3,4,5,6,7,8].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_eq!(Proofs::<Test>::get(hash), (1,0));
	});
}

#[test]
fn it_lets_user_revoke_claims() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), hash));
	});
}

#[test]
fn it_lets_user_transfer_claims() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), hash.clone(), 2));
		assert_eq!(Proofs::<Test>::get(hash), (2,0));
	});
}

#[test]
fn create_claim_error_proof_already_claimed() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), hash),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

#[test]
fn revoke_claim_error_no_such_proof() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		let another_hash: Vec<u8> = [4,5,6].to_vec();
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), another_hash),
			Error::<Test>::NoSuchProof
		);
	});
}

#[test]
fn revoke_claim_error_not_proof_owner() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), hash),
			Error::<Test>::NotProofOwner
		);
	});
}

#[test]
fn transfer_claim_error_no_such_proof() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), hash, 2),
			Error::<Test>::NoSuchProof
		);
	})
}

#[test]
fn transfer_claim_error_not_proof_owner() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), hash, 3),
			Error::<Test>::NotProofOwner
		);
	});
}
