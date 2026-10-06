//! The path a swap takes between two coins: one pair between ETH and either
//! token, and two, through WETH, between NOX and USDC.

use super::venue::{Pair, Venue};
use crate::evm::network::Chain;
use crate::net::asset::Coin;

/// One hop: the pair, the token going in and the token coming out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Hop {
    pub pair: Pair,
    pub token_in: [u8; 20],
    pub token_out: [u8; 20],
}

/// The hops from `from` to `to`, or none for a coin to itself.
pub fn route(chain: &Chain, venue: &Venue, from: Coin, to: Coin) -> Option<Vec<Hop>> {
    let token = |c: Coin| chain.token(c).unwrap_or(venue.weth);
    let pair = |c: Coin| if c == Coin::Nox { venue.nox_weth } else { venue.usdc_weth };
    let hop =
        |pair: Pair, token_in: [u8; 20], token_out: [u8; 20]| Hop { pair, token_in, token_out };
    match (from, to) {
        (a, b) if a == b => None,
        (Coin::Eth, b) => Some(vec![hop(pair(b), venue.weth, token(b))]),
        (a, Coin::Eth) => Some(vec![hop(pair(a), token(a), venue.weth)]),
        (a, b) => {
            Some(vec![hop(pair(a), token(a), venue.weth), hop(pair(b), venue.weth, token(b))])
        }
    }
}

/// The router's `path`: every token in order, WETH standing for ether.
pub fn path(hops: &[Hop]) -> Vec<[u8; 20]> {
    let mut out: Vec<[u8; 20]> = hops.first().map(|h| h.token_in).into_iter().collect();
    out.extend(hops.iter().map(|h| h.token_out));
    out
}
