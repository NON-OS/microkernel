//! Proved for every token state and every amount a transfer could carry.

use super::{outcome, Outcome, Rules};

fn any_rules() -> Rules {
    Rules {
        paused: kani::any(),
        blocked: kani::any(),
        fees: [kani::any(), kani::any(), kani::any()],
        pair_from: kani::any(),
        pair_to: kani::any(),
        exempt: kani::any(),
    }
}

/// What arrives and the fee always add up to what was sent, the fee never
/// takes all of it, and a paused or blocked token never lets a transfer through.
#[kani::proof]
#[kani::solver(cadical)]
fn what_arrives_and_the_fee_add_up_to_what_was_sent() {
    let rules = any_rules();
    let amount: u128 = kani::any();
    kani::assume(amount <= u128::from(u64::MAX) << 32);
    match outcome(&rules, amount) {
        Outcome::Arrives { amount: arrives, fee, bps } => {
            assert!(!rules.paused && !rules.blocked);
            assert_eq!(arrives.checked_add(fee), Some(amount));
            assert!(fee < amount);
            assert!(rules.fees.contains(&bps) || bps == 0);
            if rules.exempt {
                assert_eq!(fee, 0);
            }
        }
        Outcome::Refused(_) => {}
    }
}
