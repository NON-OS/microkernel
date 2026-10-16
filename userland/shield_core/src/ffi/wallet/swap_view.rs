//! A swap review as a screen shows it: every amount in its own coin's
//! decimals, percentages from basis points, and the route by name.

#[path = "swap_view_refused.rs"]
mod refused;

pub(super) use refused::refused;

use super::swap_rate::{pools, rate};
use crate::evm::swap::{Swap, SwapReview};
use crate::evm::units::{ether, format, gwei};
use crate::evm::Network;
use crate::ffi::account_types::PublicSwap;

pub(super) fn shown(
    id: u64,
    network: Network,
    swap: &Swap,
    r: &SwapReview,
    valid: u32,
) -> PublicSwap {
    let out = swap.to.decimals();
    let hops = u128::try_from(r.pairs.len()).unwrap_or(1);
    PublicSwap {
        id,
        refusal: None,
        rate: rate(swap, r.expected),
        pools: pools(swap, &r.pairs),
        pool_fee_percent: format(hops.saturating_mul(30), 2),
        approval: r.approval,
        expected_out: format(r.expected, out),
        minimum_out: format(r.minimum, out),
        token_fee: format(r.token_fee, 18),
        price_impact_percent: format(r.impact_bps, 2),
        max_network_fee: ether(r.max_network_fee),
        max_fee_per_gas_gwei: gwei(r.tx.max_fee),
        gas_limit: r.tx.gas,
        nonce: r.tx.nonce,
        valid_for_seconds: valid,
        ..refused(network, swap, "")
    }
}
