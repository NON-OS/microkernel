//! What the public account hands a screen. Amounts are text, so no shell does money math.

#[path = "account_types_swap.rs"]
mod swap;

pub use swap::PublicSwap;

use crate::evm::Network;
use crate::net::asset::Coin;

/// The account's ETH, NOX and USDC on one network.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicBalances {
    pub network: Network,
    pub address: String,
    pub eth: String,
    pub nox: String,
    pub usdc: String,
}

/// A send waiting for the user's yes. With `refusal` set, nothing is held to confirm.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicReview {
    pub id: u64,
    pub network: Network,
    pub coin: Coin,
    pub refusal: Option<String>,
    pub from: String,
    pub to: String,
    /// What leaves, and what arrives after the token's own fee, the same for ETH.
    pub amount: String,
    pub arrives: String,
    pub token_fee: String,
    pub token_fee_percent: String,
    /// The most the network can charge in ETH, then the gwei ceiling and gas limit.
    pub max_network_fee: String,
    pub max_fee_per_gas_gwei: String,
    pub gas_limit: u64,
    pub nonce: u64,
    pub chain_id: u64,
    pub to_is_contract: bool,
    pub real_money: bool,
    pub valid_for_seconds: u32,
}

/// A send the network has taken, with its block explorer link.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicSent {
    pub hash: String,
    pub link: String,
}
