#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	use scale_info::prelude::vec;



	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/* Placeholder for defining custom types. */

	// TODO: Update the `config` block below
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// TODO: Update the `event` block below
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when an account has been added
		AccountAdded(Vec<u8>, T::AccountId),
		/// Event emitted when an account has been removed
		AccountRemoved(Vec<u8>, T::AccountId),
	}

	// TODO: Update the `error` block below
	#[pallet::error]
	pub enum Error<T> {
		/// Member already exists in club
		MemberAlreadyExistInClub,
		/// Club does not exist
		ClubDoesNotExist,
		/// Member does not exist in club
		MemberDoesNotExistInClub,
	}

	#[pallet::storage]
	/// Maps each club to the members which are the accounts
	pub(super) type Clubs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		Vec<T::AccountId>,
		OptionQuery,
	>;

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(1_000)]
		pub fn add_member(
			origin: OriginFor<T>,
			club_name: Vec<u8>,
			member: T::AccountId,
		) -> DispatchResult {
			// ensure origin is a root/sudo
			ensure_root(origin)?;

			// get members
			let  members = Clubs::<T>::get(club_name.clone());

			// check if members  exists
			if members.is_some() {
				// get the known members
				let mut known_members = members.unwrap();

				// ensure member is not already part of the known members
				ensure!(!known_members.contains(&member), Error::<T>::MemberAlreadyExistInClub);

				// insert member into known members
				let _ = known_members.push(member.clone());

				Clubs::<T>::insert(club_name.clone(), known_members);
			} else {
				// create new vec of members
				let mut new_members = vec![];
				new_members.push(member.clone());

				Clubs::<T>::insert(club_name.clone(), new_members);
			}

			Self::deposit_event(Event::AccountAdded(club_name.clone(), member.clone()));

			Ok(())
		}

		#[pallet::weight(1_000)]
		pub fn remove_member(
			origin: OriginFor<T>,
			club_name: Vec<u8>,
			member: T::AccountId,
		) -> DispatchResult {
			// ensure origin is a root/sudo
			ensure_root(origin)?;

			// get members
			let  members = Clubs::<T>::get(club_name.clone());

			// ascertain that the  value exists for the club name
			ensure!(members.is_some(), Error::<T>::ClubDoesNotExist);

			let mut known_members = members.unwrap();

			// ensure member is already part of the known members
			ensure!(known_members.contains(&member), Error::<T>::MemberDoesNotExistInClub);

			// get index of member in the vec
			let index_of_member = known_members.iter().position(|x| *x == member).unwrap();

			// remove member
			known_members.remove(index_of_member);

			Clubs::<T>::insert(club_name.clone(), known_members);

			Self::deposit_event(Event::AccountRemoved(club_name.clone(), member.clone()));

			Ok(())
		}


	}
}
