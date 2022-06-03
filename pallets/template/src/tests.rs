use crate::{mock, mock::*, Error};
use frame_benchmarking::account;
use frame_support::{assert_noop, assert_ok};

const SEED: u32 = 0;

#[test]
fn should_add_new_member_to_a_new_club() {
	new_test_ext().execute_with(|| {
		let club_name = b"chelsea".to_vec();

		let account_id = account("", 1, SEED);

		assert_ok!(TemplateModule::add_member(Origin::root(), club_name.clone(), account_id));

		// assert the last event
		crate::mock::assert_last_event::<Test>(mock::Event::TemplateModule(
			crate::Event::<Test>::AccountAdded(club_name.clone(), account_id),
		));
	});
}

#[test]
fn should_remove_member_from_a_club() {
	new_test_ext().execute_with(|| {
		// first add member
		let club_name = b"chelsea".to_vec();

		let account_id = account("", 1, SEED);

		assert_ok!(TemplateModule::add_member(Origin::root(), club_name.clone(), account_id));

		// assert the last event
		crate::mock::assert_last_event::<Test>(mock::Event::TemplateModule(
			crate::Event::<Test>::AccountAdded(club_name.clone(), account_id),
		));

		// now remove member
		assert_ok!(TemplateModule::remove_member(Origin::root(), club_name.clone(), account_id));

		// assert the last event
		crate::mock::assert_last_event::<Test>(mock::Event::TemplateModule(
			crate::Event::<Test>::AccountRemoved(club_name.clone(), account_id),
		));
	});
}
