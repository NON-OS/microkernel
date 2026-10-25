//! A swap review with nothing worked out yet: the coins, the amount in and the route.

use crate::evm::swap::Swap;
use crate::evm::units::format;
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

pub(in crate::ffi::wallet) fn refused(network: Network, swap: &Swap, why: &str) -> PublicSwap {
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
