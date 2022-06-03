use super::*;
use crate as pallet_template;
use frame_support::{
	pallet_prelude::GenesisBuild,
	traits::{ConstU16, ConstU64},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_template::Config for Test {
	type Event = Event;
	type MinRegisteredClub = frame_support::traits::ConstU8<2>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let registered_club1 = (0 as u8, b"chelsea".to_vec());
	let registered_club2 = (1 as u8, b"arsenal".to_vec());

	let mut registered_clubs_vec = vec![];
	registered_clubs_vec.push(registered_club1);
	registered_clubs_vec.push(registered_club2);

	let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_template::GenesisConfig::<Test> {
		phantom: Default::default(),
		registered_clubs: Some(registered_clubs_vec),
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn assert_last_event<T: pallet_template::Config>(
	generic_event: <T as pallet_template::Config>::Event,
) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}
