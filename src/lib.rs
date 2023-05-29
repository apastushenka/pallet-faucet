#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::Currency;

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
    }

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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight((0, Pays::No))]
        pub fn mint(origin: OriginFor<T>) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            let balance = T::Currency::free_balance(&origin);
            let max_balance = T::MaxBalance::get();
            ensure!(balance < max_balance, Error::<T>::HighBalance);

            let amount = max_balance - balance;

            T::Currency::deposit_creating(&origin, amount);

            Self::deposit_event(Event::Minted {
                who: origin,
                amount,
            });

            Ok(())
        }
    }
}
