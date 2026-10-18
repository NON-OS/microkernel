//! A launch transfer that pays its relayer, and the words an eleven-word pool cannot carry.

use super::{transfer, unshield, A, FORMAT5, LAUNCH, NOX, RELAYER};
use nox_shield_core::wallet::{check_intent, Intent, IntentRefusal, PoolRules};

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
