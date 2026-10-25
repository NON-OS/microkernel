//! A withdraw on a pool with an AmountPolicy (the production pool): the fee is the policy's quote,
//! withdrawBps of the amount plus one rung, never held to MAX_FEE_BPS, which that pool does not have.

use nox_shield_core::wallet::{check_intent, Intent, IntentRefusal, PoolRules, MAX_FEE_BPS};

/// One NOX in note units (scale 1e9), and the lowest rung of the gas ladder today.
const NOX: u64 = 1_000_000_000;
const RUNG: u64 = 2_000 * NOX;
const SUBMITTER: [u8; 20] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

fn production(quote: u64) -> PoolRules {
    PoolRules { words_per_intent: 13, max_relay_fee: 0, policy_fee: Some(quote) }
}

fn withdraw(public_amount: u64, fee: u64) -> Intent {
    let held = public_amount + fee + 7 * NOX;
    Intent {
        inputs: [(held, 1, 10), (0, 1, 11)],
        outputs: [(0, 1), (7 * NOX, 1)],
        public_amount,
        fee,
        asset_id: 1,
        clearing_price: 0,
        recipient: [7; 20],
        fee_recipient: SUBMITTER,
    }
}

#[test]
fn a_production_withdraw_at_the_policys_quote_passes() {
    let p = 1_000 * NOX;
    let quote = p / 200 + RUNG;
    // 2,005 NOX on 1,000 NOX is far above 50 bps: the old cap refused every production withdraw.
    assert!(u128::from(quote) * 10_000 > u128::from(p) * MAX_FEE_BPS);
    assert_eq!(check_intent(&withdraw(p, quote), &production(quote)), Ok(()));
}

#[test]
fn a_fee_off_the_quote_is_refused() {
    let p = 1_000 * NOX;
    let quote = p / 200 + RUNG;
    for fee in [quote - 1, quote + 1, p / 200] {
        let refused = check_intent(&withdraw(p, fee), &production(quote));
        assert_eq!(refused, Err(IntentRefusal::FeeOffQuote), "fee {fee}");
    }
}

#[test]
fn without_a_policy_the_cap_still_holds() {
    let no_policy = PoolRules { policy_fee: None, ..production(0) };
    let p = 1_000 * NOX;
    assert_eq!(check_intent(&withdraw(p, p / 200), &no_policy), Ok(()));
    assert_eq!(
        check_intent(&withdraw(p, p / 200 + 1), &no_policy),
        Err(IntentRefusal::FeeExceedsCap)
    );
}
