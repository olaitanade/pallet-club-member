#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{metadata::StorageEntryModifier::Default, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::{vec, vec::Vec};
	use sp_std::default::Default as OtherDefault;

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
		type MinRegisteredClub: Get<u8>;
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
		/// Club is not up to minimum
		RegisteredClubIsNotEnough,
		/// Club not Registered
		ClubNotRegistered,
		/// Member does not exist in club
		MemberDoesNotExistInClub,
	}

	#[pallet::storage]
	#[pallet::getter(fn clubs)]
	/// Maps each club to the members which are the accounts
	pub(super) type ClubMembers<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<T::AccountId>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn registered_clubs)]
	/// Maps each club to the members which are the accounts
	pub(super) type RegisteredClubs<T: Config> =
		StorageMap<_, Blake2_128Concat, u8, Vec<u8>, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub phantom: PhantomData<T>,
		pub registered_clubs: Option<Vec<(u8, Vec<u8>)>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> OtherDefault for GenesisConfig<T> {
		fn default() -> Self {
			Self { phantom: OtherDefault::default(), registered_clubs: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(registered_clubs) = &self.registered_clubs {
				for (index, club_name) in registered_clubs {
					RegisteredClubs::<T>::insert(index, club_name);
				}
			}
		}
	}

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

			// ensure club_name exist in ClubNames storage
			let registered_club = RegisteredClubs::<T>::iter_values();

			let registered_club_count = RegisteredClubs::<T>::iter_values().count();

			ensure!(
				registered_club_count as u8 >= T::MinRegisteredClub::get(),
				Error::<T>::RegisteredClubIsNotEnough
			);

			let mut does_club_exist = false;

			for club in registered_club {
				if club_name == club {
					does_club_exist = true;
				}
			}

			ensure!(does_club_exist == true, Error::<T>::ClubNotRegistered);

			// get members
			let members = ClubMembers::<T>::get(club_name.clone());

			// check if members  exists
			if members.is_some() {
				// get the known members
				let mut known_members = members.unwrap();

				// ensure member is not already part of the known members
				ensure!(!known_members.contains(&member), Error::<T>::MemberAlreadyExistInClub);

				// insert member into known members
				let _ = known_members.push(member.clone());

				ClubMembers::<T>::insert(club_name.clone(), known_members);
			} else {
				// create new vec of members
				let mut new_members = vec![];
				new_members.push(member.clone());

				ClubMembers::<T>::insert(club_name.clone(), new_members);
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

			// ensure club_name exist in ClubNames storage
			let registered_club = RegisteredClubs::<T>::iter_values();

			let registered_club_count = RegisteredClubs::<T>::iter_values().count();

			ensure!(
				registered_club_count as u8 >= T::MinRegisteredClub::get(),
				Error::<T>::RegisteredClubIsNotEnough
			);

			let mut does_club_exist = false;

			for club in registered_club {
				if club_name == club {
					does_club_exist = true;
				}
			}

			ensure!(does_club_exist == true, Error::<T>::ClubNotRegistered);

			// get members
			let members = ClubMembers::<T>::get(club_name.clone());

			// ascertain that the  value exists for the club name
			ensure!(members.is_some(), Error::<T>::ClubDoesNotExist);

			let mut known_members = members.unwrap();

			// ensure member is already part of the known members
			ensure!(known_members.contains(&member), Error::<T>::MemberDoesNotExistInClub);

			// get index of member in the vec
			let index_of_member = known_members.iter().position(|x| *x == member).unwrap();

			// remove member
			known_members.remove(index_of_member);

			ClubMembers::<T>::insert(club_name.clone(), known_members);

			Self::deposit_event(Event::AccountRemoved(club_name.clone(), member.clone()));

			Ok(())
		}
	}
}
