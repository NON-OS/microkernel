//! Value and assets must balance over the integers, every value in range.

use super::{transfer, unshield, A, FORMAT5, LAUNCH, NOX};
use nox_shield_core::notes::MAX_VALUE;
use nox_shield_core::wallet::{check_intent, Intent, IntentRefusal};

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
