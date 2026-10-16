//! The batch a send's review reads, from two servers that must agree when it is NOX.

use super::super::network::Chain;
use super::super::reply::Answer;
use super::super::review_checks;
use super::super::rpc::{ask, ask_each, Call};
use crate::error::NetError;
use crate::net::asset::Coin;
use crate::net::tor::Tor;

/// The answers to `calls`, or the refusal when two servers disagree about NOX.
pub(super) fn answers(
    tor: &Tor,
    chain: &Chain,
    coin: Coin,
    calls: &[Call],
) -> Result<Result<Vec<Answer>, &'static str>, NetError> {
    // A NOX review rests on the token's fees and flags, so they are read
    // from two servers and must agree. Ether's amount is exact, and USDC
    // takes no fee, so one server's answer is enough for those.
    Ok(Ok(match coin {
        Coin::Eth | Coin::Usdc => ask(tor, chain, calls)?,
        Coin::Nox => {
            let mut both = ask_each(tor, chain, calls, 2)?.into_iter();
            let (first, second) = (both.next(), both.next());
            let (Some(first), Some(second)) = (first, second) else {
                return Err(NetError::ReplyShape);
            };
            if !review_checks::token_agrees(chain, &first, &second)? {
                return Ok(Err(
                    "Two servers disagree about NOX's fees. Nothing was sent; try again.",
                ));
            }
            first
        }
    }))
}
