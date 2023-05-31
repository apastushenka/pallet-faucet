#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, OnKilledAccount};

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// The maximum balance of an account
        #[pallet::constant]
        type MaxBalance: Get<BalanceOf<Self>>;

        /// The minimum interval in blocks between mints
        #[pallet::constant]
        type MinInterval: Get<Self::BlockNumber>;
    }

    /// The last mint timestamp for an account
    #[pallet::storage]
    pub type LastMint<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, T::BlockNumber, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        Minted {
            who: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Balance exceed `MaxBalance`
        HighBalance,

        /// `MinInterval` had not passed since the last mint
        RecentlyMinted,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight((0, Pays::No))]
        pub fn mint(origin: OriginFor<T>) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            let current_block = frame_system::Pallet::<T>::block_number();
            if let Some(minted_at) = LastMint::<T>::get(&origin) {
                ensure!(
                    current_block >= minted_at + T::MinInterval::get(),
                    Error::<T>::RecentlyMinted
                );
            }

            let balance = T::Currency::free_balance(&origin);
            let max_balance = T::MaxBalance::get();
            ensure!(balance < max_balance, Error::<T>::HighBalance);

            let amount = max_balance - balance;

            T::Currency::deposit_creating(&origin, amount);

            LastMint::<T>::insert(&origin, current_block);

            Self::deposit_event(Event::Minted {
                who: origin,
                amount,
            });

            Ok(())
        }
    }

    impl<T: Config> OnKilledAccount<T::AccountId> for Pallet<T> {
        fn on_killed_account(who: &T::AccountId) {
            LastMint::<T>::remove(who);
        }
    }
}
