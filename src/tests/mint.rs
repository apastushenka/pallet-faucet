use crate::{tests::mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};

#[test]
fn mint() {
    ExtBuilder::default()
        .balances(vec![(ALICE, EXISTENTIAL_DEPOSIT)])
        .build()
        .execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Faucet::mint(RuntimeOrigin::signed(ALICE)));
            assert_eq!(Balances::free_balance(ALICE), MAX_BALANCE);

            let amount = MAX_BALANCE - EXISTENTIAL_DEPOSIT;
            System::assert_last_event(Event::Minted { who: ALICE, amount }.into())
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
