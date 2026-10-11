//! Gas past the kind's limit, or a fee past the meter's caps, is refused
//! before any key is derived (v1.4.51).

use super::super::*;
use super::{ask, refund_data, refused, FRESH};
use nonos_nox_config::{MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS, REFUND_GAS};
use FreshRefusal::FeesPastCaps;

#[test]
fn fees_past_the_caps_are_refused() {
    let r = ask(refund_data(FRESH));
    for past in [
        FreshSign { gas: REFUND_GAS + 1, ..r.clone() },
        FreshSign { gas: 0, ..r.clone() },
        FreshSign { max_fee: MAX_FEE_PER_GAS + 1, ..r.clone() },
        FreshSign { max_fee: 0, max_priority: 0, ..r.clone() },
        FreshSign {
            max_priority: MAX_PRIORITY_FEE_PER_GAS + 1,
            max_fee: MAX_FEE_PER_GAS,
            ..r.clone()
        },
        FreshSign { max_priority: 30_000_000_000, max_fee: 20_000_000_000, ..r.clone() },
    ] {
        assert_eq!(refused(&past, Some(4)), FeesPastCaps, "{past:?}");
    }
}
