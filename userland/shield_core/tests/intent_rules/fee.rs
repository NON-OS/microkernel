//! An unshield's fee against the cap, and the public words a transfer may not carry.

use super::{transfer, unshield, FORMAT5};
use nox_shield_core::wallet::{check_intent, IntentRefusal};

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
