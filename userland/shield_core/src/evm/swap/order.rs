//! A swap as the user asked for it, and what its review hands back.

use crate::evm::tx::Eip1559;
use crate::net::asset::Coin;

/// Swap `amount` base units of `from` for `to`, accepting at most
/// `slippage_bps` less than the quote.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Swap {
    pub from: Coin,
    pub to: Coin,
    pub amount: u128,
    pub slippage_bps: u16,
}

/// A swap ready to sign, or the approval it needs first.
pub struct SwapReview {
    pub tx: Eip1559,
    /// This transaction lets the router take only `amount` of the coin in, and
    /// the swap is reviewed again once it is mined.
    pub approval: bool,
    /// What the quote says arrives, the least the swap will accept, and the
    /// fee the NOX token takes on the way, in its own units.
    pub expected: u128,
    pub minimum: u128,
    pub token_fee: u128,
    pub impact_bps: u128,
    pub max_network_fee: u128,
    /// The pairs the swap passes through, in order.
    pub pairs: Vec<[u8; 20]>,
}

pub enum SwapChecked {
    Ready(Box<SwapReview>),
    Refused(&'static str),
}
