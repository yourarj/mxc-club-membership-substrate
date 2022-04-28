#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// mocks for test
#[cfg(test)]
mod mock;

// tests
#[cfg(test)]
mod tests;

// benchmarking the pallet
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// ### MxcClub Pallet Struct
	/// pallet struct of Greeter
	/// business logic & callable gather around it
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// as mxcclub pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// For constraining the member length
		type MaxBytesInMemberName: Get<u32>;
	}

	/// ### A Member

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]

	pub struct Member<T: Config> {
		pub id: T::AccountId,
		pub name: BoundedVec<u8, T::MaxBytesInMemberName>,
	}

	// StorageMap containing entries of AccountId and Member
	// object defined above
	#[pallet::storage]
	pub(super) type Members<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Member<T>>;

	// events this pallet will
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// when new member is added
		MemberAdded { member: T::AccountId },

		// when member is removed
		MemberRemoved { member: T::AccountId },
	}

	// error to report if case of undesired situation
	#[pallet::error]
	pub enum Error<T> {
		// When specified member is not found
		MemberNotFound,
	}

	// Pallet's callables.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// let owner origins add member in club
		#[pallet::weight(0)]
		pub fn add_member(
			origin: OriginFor<T>,
			member_account_id: T::AccountId,
			member_name: BoundedVec<u8, T::MaxBytesInMemberName>,
		) -> DispatchResult {
			// make sure the it's root
			ensure_root(origin.clone())?;

			// Make sure the caller is from a signed origin
			// let _user = ensure_signed(origin)?;

			// Generate unique DNA and Gender using a helper function
			Self::do_add_member(member_account_id, member_name)?;
			Ok(())
		}

		/// let root remove member
		#[pallet::weight(0)]
		pub fn remove_member(
			origin: OriginFor<T>,
			member_account_id: T::AccountId,
		) -> DispatchResult {
			// make sure the it's root
			ensure_root(origin.clone())?;

			// Make sure the caller is from a signed origin
			// let _user = ensure_signed(origin)?;

			Self::do_remove_member(member_account_id)?;
			Ok(())
		}
	}

	// methods internal to pallet
	impl<T: Config> Pallet<T> {
		// add member in club's onchain stoarge
		fn do_add_member(
			member_account_id: T::AccountId,
			member_name: BoundedVec<u8, T::MaxBytesInMemberName>,
		) -> Result<(), DispatchError> {
			// create new member
			let new_member = Member { id: member_account_id.clone(), name: member_name };

			// add in clubs on chain storage
			Members::<T>::insert(&member_account_id, new_member);

			// emit member addition event
			Self::deposit_event(Event::MemberAdded { member: member_account_id.clone() });

			// return ok
			Ok(())
		}

		// remove member from on chain storage
		fn do_remove_member(member_account_id: T::AccountId) -> Result<(), DispatchError> {
			if !Members::<T>::contains_key(&member_account_id) {
				return Err(Error::<T>::MemberNotFound.into());
			}

			// update members
			Members::<T>::remove(&member_account_id);

			// emit the event
			Self::deposit_event(Event::MemberRemoved { member: member_account_id.clone() });

			// return ok
			Ok(())
		}
	}
}
