//! What a review reads for the coin being sent, and what the answers say arrives.
//! NOX: the token's state and a simulated transfer, then the balance. USDC: a
//! simulated transfer as the sender, then the balance. Ether needs nothing more.

use super::calls::call;
use super::network::Chain;
use super::nox::{self, BALANCE_OF, READS};
use super::reply::Answer;
use super::review_checks::{self, COMMON};
use super::review_parts::{Arrival, Order};
use super::rpc::Call;
use crate::error::NetError;
use crate::net::asset::Coin;

pub(super) fn calls(chain: &Chain, from: &[u8; 20], order: &Order) -> Vec<Call> {
    let balance = |token: &[u8; 20]| call(None, token, &nox::with_address(BALANCE_OF, from));
    match order.coin {
        Coin::Eth => Vec::new(),
        Coin::Nox => {
            let mut out = nox::reads(chain, from, &order.to, order.amount);
            out.push(balance(&chain.nox_token));
            out
        }
        Coin::Usdc => {
            let usdc = &chain.usdc_token;
            vec![call(Some(from), usdc, &nox::transfer(&order.to, order.amount)), balance(usdc)]
        }
    }
}

pub(super) fn arrival(
    chain: &Chain,
    order: &Order,
    answers: &[Answer],
) -> Result<Result<Arrival, &'static str>, NetError> {
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    match order.coin {
        Coin::Eth => Ok(Ok(Arrival { arrives: order.amount, fee: 0, bps: 0 })),
        Coin::Nox => {
            let token = answers.get(COMMON..COMMON.saturating_add(READS)).unwrap_or_default();
            let held = at(COMMON.saturating_add(READS))?.word()?;
            review_checks::nox_arrival(chain, token, held, order.amount)
        }
        Coin::Usdc => {
            let held = at(COMMON.saturating_add(1))?.word()?;
            Ok(review_checks::usdc_arrival(at(COMMON)?, held, order.amount))
        }
    }
}
