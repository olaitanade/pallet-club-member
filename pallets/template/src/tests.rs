use frame_benchmarking::account;
use frame_benchmarking::baseline::mock::new_test_ext;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const SEED: U32 = 0;

#[test]
fn should_add_new_member_to_a_new_club() {
	new_test_ext().execute_with(|| {
		let club_name = b"chelsea".to_vec();

		let account_id = account::<AccountId>("", 1, SEED);

		assert_ok!(TemplateModule::add_member(Origin::root(), club_name, account_id));

		// assert the last event
		assert_last_event::<Test>(crate::Event::AccountAdded(club_name.clone(), account_id));
	});
}


#[test]
fn should_remove_member_from_a_club() {
	new_test_ext().execute_with(|| {
		let club_name = b"chelsea".to_vec();

		let account_id = account::<AccountId>("", 1, SEED);

		assert_ok!(TemplateModule::add_member(Origin::root(), club_name, account_id));

		// assert the last event
		assert_last_event::<Test>(crate::Event::AccountAdded(club_name.clone(), account_id));
	});
}
