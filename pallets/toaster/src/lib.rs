#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	const NUMBER_OF_SLOTS: u8 = 4;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
    #[pallet::getter(fn number_of)]
	pub type Taken<T> = StorageValue<_, u8>;

	#[pallet::storage]
    #[pallet::getter(fn account_id_with_slot)]
	pub type AccountIdWithSlot<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (), ValueQuery>;
	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Slot reserved by Account with id AccountId
		SlotTaken(T::AccountId),
		/// All toasts are cooked
		Toasted(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Not all of the slots are taken
		NotReady,
		/// Double reservation of slots
		DoubleReservation,
		/// No slots available
		AllSlotsTaken,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Put some bread into a toaster slot, effectively reserving it
		/// /// only 2 read_writes as already accessed and written into value count only once
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn reserve_slot(origin: OriginFor<T>)  -> DispatchResult  {
			let account_id = ensure_signed(origin)?;
			// We only have 4 slots
			ensure!(<Taken<T>>::get() != Some(NUMBER_OF_SLOTS), Error::<T>::AllSlotsTaken);
			// In case Account have already reserved the slot, one shouldn't allow to reserve another one, therefore return the DoubleReservation error
			ensure!(!AccountIdWithSlot::<T>::contains_key(&account_id), Error::<T>::DoubleReservation);
			// Insert the bread
			AccountIdWithSlot::<T>::insert(account_id.clone(), ());
			// Increase counter
			<Taken<T>>::mutate(|v| if let Some(ref mut s) = *v {
					*s += 1;
				} else {
					*v = Some(1u8);
				}
			);
			Self::deposit_event(Event::SlotTaken(account_id));
            Ok(())
		}

		/// Do the cooking
		/// only 2 read_writes as already accessed and written into value count only once
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn cook(origin: OriginFor<T>)  -> DispatchResult  {
			// Should cook, only when the taoster is full, otherwise should return the NotReady error
			ensure!(<Taken<T>>::get() == Some(NUMBER_OF_SLOTS), Error::<T>::NotReady);
			// Send an event about each account, and take out the cooked bread out of the slots
			for account_id in AccountIdWithSlot::<T>::drain() {
				Self::deposit_event(Event::SlotTaken(account_id.0));
			}
			// Ensure that the number of taken slots corresponds to the same number in the map
			<Taken<T>>::put(0);
            Ok(())
		}
	}
}
