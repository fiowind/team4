// Tests to be written here

use crate::{Error, Proofs, mock::*};
use frame_support::{assert_ok, assert_noop, StorageMap};

#[test]
fn it_lets_user_create_claims() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_eq!(Proofs::<Test>::get(hash), (1,0));
	});
}

#[test]
fn correct_error_for_already_claimed() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), hash.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}
