use crate::{Error, RawEvent, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_and_revoke() {
    new_test_ext().execute_with(|| {
        let claim = vec![1u8, 1];
        assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Poe::get_proof(claim.clone()), Some((1, 0)));
        assert_noop!(
			Poe::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
        assert_noop!(
			Poe::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
        assert_noop!(
			Poe::revoke_claim(Origin::signed(2), vec![2, 2]),
			Error::<Test>::ProofNotExist
		);
        assert_ok!(Poe::revoke_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
			Poe::revoke_claim(Origin::signed(1), claim),
			Error::<Test>::ProofNotExist
		);
        assert_noop!(
            Poe::create_claim(Origin::signed(1), vec![0u8; 1024]),
            Error::<Test>::TooLong
        );
        assert_noop!(
            Poe::create_claim(Origin::signed(1), vec![0u8; 0]),
            Error::<Test>::TooShort
        );
    });
}

#[test]
fn transfer() {
    ExtBuilder::build().execute_with(|| {
        let claim = vec![1u8, 1];
        assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(Poe::transfer_claim(Origin::signed(1), claim.clone(), 2));
        let expected_event = TestEvent::generic_event(RawEvent::ClaimTransferred(1, 2, claim.clone(), 1));
        assert!(System::events().iter().any(|a| a.event == expected_event));
        assert_eq!(Poe::get_proof(claim.clone()), Some((2, 1)));
    });
}
