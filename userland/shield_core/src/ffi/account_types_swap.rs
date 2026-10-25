//! What the public account hands a screen for a swap.

use crate::evm::Network;
use crate::net::asset::Coin;

/// A swap waiting for a yes, or why not. With `approval` set, this transaction only
/// lets the router take `amount_in`, and the swap is reviewed again once mined.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicSwap {
    pub id: u64,
    pub network: Network,
    pub from: Coin,
    pub to: Coin,
    pub refusal: Option<String>,
    pub approval: bool,
    /// What goes in, what the quote says arrives, and the least the swap accepts.
    pub amount_in: String,
    pub expected_out: String,
    pub minimum_out: String,
    /// The NOX token's own fee on the way, in NOX, "0" when none.
    pub token_fee: String,
    pub price_impact_percent: String,
    pub slippage_percent: String,
    /// The coins in order, as a screen names them: "NOX → WETH → USDC".
    pub route: String,
    pub rate: String,
    pub venue: String,
    pub pools: Vec<String>,
    pub pool_fee_percent: String,
    pub max_network_fee: String,
    pub max_fee_per_gas_gwei: String,
    pub gas_limit: u64,
    pub nonce: u64,
    pub chain_id: u64,
    pub real_money: bool,
    pub valid_for_seconds: u32,
}
