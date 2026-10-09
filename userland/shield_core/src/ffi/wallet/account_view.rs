//! A review as a screen shows it: every amount formatted, the token's fee as
//! a percentage, and the real-money flag on mainnet.

use crate::evm::units::{ether, format, gwei};
use crate::evm::{checksummed, Network, Order, Review};
use crate::ffi::account_types::PublicReview;
use std::time::Duration;

/// How long a review stays confirmable: long enough to read it, short enough
/// that its fees and nonce are still the network's.
pub(super) const VALID_FOR: Duration = Duration::from_secs(90);

pub(super) fn view(
    id: u64,
    network: Network,
    from: &[u8; 20],
    order: &Order,
    review: &Review,
) -> PublicReview {
    let bps = review.arrival.bps;
    PublicReview {
        id,
        refusal: None,
        arrives: format(review.arrival.arrives, order.coin.decimals()),
        token_fee: format(review.arrival.fee, order.coin.decimals()),
        token_fee_percent: crate::evm::units::format(u128::from(bps), 2),
        max_network_fee: ether(review.max_network_fee),
        max_fee_per_gas_gwei: gwei(review.tx.max_fee),
        gas_limit: review.tx.gas,
        nonce: review.tx.nonce,
        chain_id: review.tx.chain_id,
        to_is_contract: review.to_is_contract,
        valid_for_seconds: u32::try_from(VALID_FOR.as_secs()).unwrap_or(0),
        ..refusal(network, from, order, "")
    }
}

/// A review that holds nothing, and why.
pub(super) fn refusal(network: Network, from: &[u8; 20], order: &Order, why: &str) -> PublicReview {
    PublicReview {
        id: 0,
        network,
        coin: order.coin,
        refusal: Some(why.to_string()),
        from: checksummed(from),
        to: checksummed(&order.to),
        amount: format(order.amount, order.coin.decimals()),
        arrives: String::new(),
        token_fee: String::new(),
        token_fee_percent: String::new(),
        max_network_fee: String::new(),
        max_fee_per_gas_gwei: String::new(),
        gas_limit: 0,
        nonce: 0,
        chain_id: network.chain().chain_id,
        to_is_contract: false,
        real_money: network == Network::Mainnet,
        valid_for_seconds: 0,
    }
}
