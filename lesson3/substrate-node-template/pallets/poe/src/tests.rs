// Tests to be written here

use super::*;
use crate::{Error,mock::*};

use frame_support::{assert_ok,assert_noop};

//teset cases for create_claim
#[test]
fn create_claim_works(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1,3,4];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),(1,system::Module::<Test>::block_number()));
	})
}


