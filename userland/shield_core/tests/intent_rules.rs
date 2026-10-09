//! The spend rules, each against the pool's own refusal.
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used, clippy::arithmetic_side_effects)]

use nox_shield_core::notes::MAX_VALUE;
use nox_shield_core::wallet::{check_intent, Intent, IntentRefusal, PoolRules};

/// The format 5 pool: eleven words, no relay fee.
const FORMAT5: PoolRules = PoolRules { words_per_intent: 11, max_relay_fee: 0 };
/// A launch pool: twelve words, transfers may pay a relayer up to 1,000 units.
const LAUNCH: PoolRules = PoolRules { words_per_intent: 12, max_relay_fee: 1_000 };
const RELAYER: [u8; 20] = [9; 20];

const NOX: u64 = 1_000_000_000_000_000_000;
const A: u64 = 1;

fn transfer() -> Intent {
    // 3 NOX in, 2 to the payee, 1 back as change.
    Intent {
        inputs: [(2 * NOX, A, 10), (NOX, A, 11)],
        outputs: [(2 * NOX, A), (NOX, A)],
        public_amount: 0,
        fee: 0,
        asset_id: A,
        clearing_price: 0,
        recipient: [0; 20],
        fee_recipient: [0; 20],
    }
}

fn unshield(fee: u64) -> Intent {
    // 3 NOX in, 1 leaves the pool to a recipient, fee taken from it, rest as change.
    let public_amount = 1_000_000;
    Intent {
        inputs: [(2_000_000, A, 10), (1_000_000, A, 11)],
        outputs: [(2_000_000 - fee, A), (0, A)],
        public_amount,
        fee,
        asset_id: A,
        clearing_price: 0,
        recipient: [7; 20],
        fee_recipient: [0; 20],
    }
}

#[test]
fn a_plain_transfer_and_an_unshield_at_the_cap_pass() {
    assert_eq!(check_intent(&transfer(), &FORMAT5), Ok(()));
    // 0.5% of 1,000,000 is 5,000, and a fee at the cap is allowed.
    assert_eq!(check_intent(&unshield(5_000), &FORMAT5), Ok(()));
}

#[test]
fn the_pools_own_example_is_refused() {
    // docs/16: a fee of 25,000 on an unshield of 1,000,000 is 250 bps.
    assert_eq!(check_intent(&unshield(25_000), &FORMAT5), Err(IntentRefusal::FeeExceedsCap));
    assert_eq!(check_intent(&unshield(5_001), &FORMAT5), Err(IntentRefusal::FeeExceedsCap));
}

#[test]
fn a_transfer_with_a_public_word_is_refused() {
    let mut t = transfer();
    t.recipient = [1; 20];
    assert_eq!(check_intent(&t, &FORMAT5), Err(IntentRefusal::TransferHasPublicLeg));
    let mut t = transfer();
    t.clearing_price = 1;
    assert_eq!(
        check_intent(&t, &FORMAT5),
        Err(IntentRefusal::TransferHasPublicLeg),
        "stricter than the pool"
    );
}

#[test]
fn an_unshield_to_nobody_is_refused() {
    let mut u = unshield(0);
    u.recipient = [0; 20];
    assert_eq!(check_intent(&u, &FORMAT5), Err(IntentRefusal::RecipientRequired));
}

#[test]
fn value_and_assets_must_balance_over_the_integers() {
    let mut t = transfer();
    t.outputs[0].0 += 1;
    assert_eq!(check_intent(&t, &FORMAT5), Err(IntentRefusal::Unbalanced), "one base unit created");
    let mut t = transfer();
    t.outputs[1].1 = 0;
    assert_eq!(check_intent(&t, &FORMAT5), Err(IntentRefusal::MixedAssets));
    let mut t = transfer();
    t.inputs[1].2 = 10;
    assert_eq!(check_intent(&t, &FORMAT5), Err(IntentRefusal::SameInputTwice));
}

#[test]
fn the_modular_inflation_shape_is_refused() {
    // Proposition 4.1 in the STARK docs: outputs 2 + K and p - K balance inputs
    // 1 and 1 modulo p. Over the integers they do not.
    let p = 0xFFFF_FFFF_0000_0001u64;
    let k = NOX;
    let t =
        Intent { inputs: [(1, A, 1), (1, A, 2)], outputs: [(2 + k, A), (p - k, A)], ..transfer() };
    assert_eq!(check_intent(&t, &FORMAT5), Err(IntentRefusal::Unbalanced));
}

/// A launch transfer that pays its relayer, as the pool now allows.
fn relayed_transfer(fee: u64) -> Intent {
    Intent { outputs: [(2 * NOX - fee, A), (NOX, A)], fee, fee_recipient: RELAYER, ..transfer() }
}

#[test]
fn a_launch_transfer_may_pay_its_relayer_up_to_the_cap() {
    assert_eq!(check_intent(&relayed_transfer(1_000), &LAUNCH), Ok(()));
    assert_eq!(check_intent(&relayed_transfer(1_001), &LAUNCH), Err(IntentRefusal::FeeExceedsCap));
    // The same transfer on a pool whose cap is zero pays nothing.
    let no_fee = PoolRules { max_relay_fee: 0, ..LAUNCH };
    assert_eq!(check_intent(&relayed_transfer(1), &no_fee), Err(IntentRefusal::FeeExceedsCap));
}

#[test]
fn a_fee_and_its_recipient_come_together() {
    let mut t = transfer();
    t.fee_recipient = RELAYER;
    assert_eq!(check_intent(&t, &LAUNCH), Err(IntentRefusal::FeeRecipientWithoutFee));
    let mut t = relayed_transfer(500);
    t.fee_recipient = [0; 20];
    assert_eq!(check_intent(&t, &LAUNCH), Err(IntentRefusal::FeeWithoutFeeRecipient));
}

#[test]
fn an_eleven_word_pool_has_no_fee_recipient_word() {
    assert_eq!(check_intent(&relayed_transfer(1), &FORMAT5), Err(IntentRefusal::NotRepresentable));
}

#[test]
fn an_eleven_word_pool_refuses_an_address_with_a_limb_at_p_and_twelve_takes_it() {
    let mut u = unshield(0);
    u.recipient = [0xFF; 20];
    assert_eq!(check_intent(&u, &FORMAT5), Err(IntentRefusal::NotRepresentable));
    assert_eq!(check_intent(&u, &LAUNCH), Ok(()));
}

#[test]
fn values_above_max_value_are_refused() {
    let t = Intent {
        inputs: [(MAX_VALUE + 1, A, 1), (0, A, 2)],
        outputs: [(MAX_VALUE + 1, A), (0, A)],
        ..transfer()
    };
    assert_eq!(check_intent(&t, &LAUNCH), Err(IntentRefusal::OutOfRange));
    let at = Intent {
        inputs: [(MAX_VALUE, A, 1), (0, A, 2)],
        outputs: [(MAX_VALUE, A), (0, A)],
        ..transfer()
    };
    assert_eq!(check_intent(&at, &LAUNCH), Ok(()));
    let mut u = unshield(0);
    u.clearing_price = 0xFFFF_FFFF_0000_0001;
    assert_eq!(check_intent(&u, &LAUNCH), Err(IntentRefusal::OutOfRange));
}
