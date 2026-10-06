//! A swap review as a screen shows it: every amount in its own coin's
//! decimals, percentages from basis points, and the route by name.

use super::swap_rate::{pools, rate};
use crate::evm::swap::{Swap, SwapReview};
use crate::evm::units::{ether, format, gwei};
use crate::evm::Network;
use crate::ffi::account_types::PublicSwap;
use crate::net::asset::Coin;

fn route(swap: &Swap) -> String {
    let through = swap.from != Coin::Eth && swap.to != Coin::Eth;
    if through {
        format!("{} → WETH → {}", swap.from.symbol(), swap.to.symbol())
    } else {
        format!("{} → {}", swap.from.symbol(), swap.to.symbol())
    }
}

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

pub(super) fn refused(network: Network, swap: &Swap, why: &str) -> PublicSwap {
    PublicSwap {
        id: 0,
        network,
        from: swap.from,
        to: swap.to,
        refusal: Some(why.to_string()),
        approval: false,
        amount_in: format(swap.amount, swap.from.decimals()),
        expected_out: String::new(),
        minimum_out: String::new(),
        token_fee: String::new(),
        price_impact_percent: String::new(),
        slippage_percent: format(u128::from(swap.slippage_bps), 2),
        route: route(swap),
        rate: String::new(),
        venue: "Uniswap V2".to_string(),
        pools: Vec::new(),
        pool_fee_percent: String::new(),
        max_network_fee: String::new(),
        max_fee_per_gas_gwei: String::new(),
        gas_limit: 0,
        nonce: 0,
        chain_id: network.chain().chain_id,
        real_money: network == Network::Mainnet,
        valid_for_seconds: 0,
    }
}
