use crate::{tests::mock::*, Error, Event, LastMint};
use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::Currency};

#[test]
fn mint() {
    ExtBuilder::default()
        .balances(vec![(ALICE, EXISTENTIAL_DEPOSIT)])
        .build()
        .execute_with(|| {
            let block = 1;
            System::set_block_number(block);

            assert_ok!(Faucet::mint(RuntimeOrigin::signed(ALICE)));
            assert_eq!(Balances::free_balance(ALICE), MAX_BALANCE);
            assert_eq!(LastMint::<TestRuntime>::get(ALICE), Some(block));

            let amount = MAX_BALANCE - EXISTENTIAL_DEPOSIT;
            System::assert_last_event(Event::Minted { who: ALICE, amount }.into())
        })
}

#[test]
fn mint_twice() {
    ExtBuilder::default().build().execute_with(|| {
        let _ = Faucet::mint(RuntimeOrigin::signed(ALICE));
        let _ = Balances::slash(&ALICE, 1);

        let block = MIN_INTERVAL;
        System::set_block_number(block);

        assert_ok!(Faucet::mint(RuntimeOrigin::signed(ALICE)));
        assert_eq!(Balances::free_balance(ALICE), MAX_BALANCE);
        assert_eq!(LastMint::<TestRuntime>::get(ALICE), Some(block));

        System::assert_last_event(
            Event::Minted {
                who: ALICE,
                amount: 1,
            }
            .into(),
        )
    })
}

#[test]
fn must_be_signed() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(Faucet::mint(RuntimeOrigin::none()), BadOrigin);
    })
}

#[test]
fn high_balance() {
    ExtBuilder::default()
        .balances(vec![(ALICE, MAX_BALANCE)])
        .build()
        .execute_with(|| {
            assert_noop!(
                Faucet::mint(RuntimeOrigin::signed(ALICE)),
                Error::<TestRuntime>::HighBalance
            );
        })
}

#[test]
fn short_interval() {
    ExtBuilder::default().build().execute_with(|| {
        let _ = Faucet::mint(RuntimeOrigin::signed(ALICE));
        let _ = Balances::slash(&ALICE, 1);

        System::set_block_number(MIN_INTERVAL - 1);

        assert_noop!(
            Faucet::mint(RuntimeOrigin::signed(ALICE)),
            Error::<TestRuntime>::RecentlyMinted
        );
    })
}
