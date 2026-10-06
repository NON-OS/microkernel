//! Proved for every base fee, suggested tip and estimate, not sampled.

use super::{limit, offer, CEILING, MAX_TIP};

/// Whatever an RPC says, an offer never passes the ceiling or the tip cap,
/// and its tip never exceeds its fee cap, which EIP-1559 requires of a
/// transaction the chain will take.
#[kani::proof]
fn an_offer_is_always_one_the_chain_accepts_and_never_overpays() {
    let (base, suggested): (u128, u128) = (kani::any(), kani::any());
    if let Some(f) = offer(base, suggested) {
        assert!(f.max_fee <= CEILING);
        assert!(f.max_priority_fee <= MAX_TIP);
        assert!(f.max_priority_fee <= f.max_fee);
        assert!(f.max_fee >= base);
    }
}

/// Ether to a person gets 21,000 gas, and anything else never gets less gas
/// than the estimate asked for.
#[kani::proof]
fn a_gas_limit_is_exact_for_ether_and_never_below_the_estimate() {
    let estimate: u128 = kani::any();
    assert_eq!(limit(estimate, true), Some(21_000));
    if let Some(gas) = limit(estimate, false) {
        assert!(u128::from(gas) >= estimate);
    }
}
