//! A swap, worked out in full before anything is signed: the route, the
//! market read from two servers, the quote with NOX's own rules, the price
//! impact, the minimum that must arrive, and then either the exact approval
//! it needs first or the swap itself, simulated as the sender.

use super::order::{Swap, SwapChecked};
use super::{build, parse, reads, route};
use crate::error::NetError;
use crate::evm::network::Network;
use crate::evm::rpc::ask_each;
use crate::net::asset::Coin;
use crate::net::tor::Tor;

pub fn review_swap(
    tor: &Tor,
    network: Network,
    from: &[u8; 20],
    swap: &Swap,
    sends: &[(u64, [u8; 32])],
) -> Result<SwapChecked, NetError> {
    let chain = network.chain();
    let Some(venue) = chain.swaps else {
        return Ok(SwapChecked::Refused("Swaps run on Ethereum mainnet, where the pools are."));
    };
    if let Some(why) = super::checks::before_reading(swap) {
        return Ok(SwapChecked::Refused(why));
    }
    let Some(hops) = route::route(&chain, &venue, swap.from, swap.to) else {
        return Ok(SwapChecked::Refused("Choose two different coins."));
    };
    let (token_in, nox) = (swap.from != Coin::Eth, swap.from == Coin::Nox || swap.to == Coin::Nox);
    let calls = reads::calls(&chain, &venue, from, &hops, nox);
    let mut both = ask_each(tor, &chain, &calls, 2)?.into_iter();
    let (Some(first), Some(second)) = (both.next(), both.next()) else {
        return Err(NetError::ReplyShape);
    };
    let one = parse::market(&chain, &hops, token_in, nox, &first)?;
    let two = parse::market(&chain, &hops, token_in, nox, &second)?;
    let (market, other) = match (one, two) {
        (Ok(m), Ok(o)) => (m, o),
        (Err(why), _) | (_, Err(why)) => return Ok(SwapChecked::Refused(why)),
    };
    if !super::checks::agree(&market, &other) {
        return Ok(SwapChecked::Refused(
            "Two servers disagree about the pools. Nothing was sent; try again.",
        ));
    }
    let plan = match super::checks::plan(&hops, &market, swap) {
        Ok(plan) => plan,
        Err(why) => return Ok(SwapChecked::Refused(why)),
    };
    build::finish(tor, &chain, &venue, from, swap, &hops, &first, plan, sends).map(|ready| {
        match ready {
            Ok(review) => SwapChecked::Ready(Box::new(review)),
            Err(why) => SwapChecked::Refused(why),
        }
    })
}
