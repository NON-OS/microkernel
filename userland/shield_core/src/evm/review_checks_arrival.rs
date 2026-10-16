//! What a token transfer delivers, read from the token's own answers, or why it would not.

use super::super::network::Chain;
use super::super::nox::READS;
use super::super::nox_revert::why;
use super::super::nox_rules::{outcome, Outcome};
use super::super::reply::Answer;
use super::super::review_parts::Arrival;
use super::COMMON;
use crate::error::NetError;

/// What a NOX transfer delivers, or why it would not happen.
pub(in crate::evm) fn nox_arrival(
    chain: &Chain,
    token: &[Answer],
    held: u128,
    amount: u128,
) -> Result<Result<Arrival, &'static str>, NetError> {
    if token.len() != READS {
        return Err(NetError::ReplyShape);
    }
    let Some(rules) = super::super::nox_read::rules(chain, token)? else {
        return Ok(Err(
            "NOX's contract changed after this app was built. Update the app to send NOX.",
        ));
    };
    if held < amount {
        return Ok(Err("The NOX balance does not cover that."));
    }
    if let Some(Answer::Reverted(data)) = token.last() {
        return Ok(Err(why(data)));
    }
    Ok(match outcome(&rules, amount) {
        Outcome::Arrives { amount, fee, bps } => Ok(Arrival { arrives: amount, fee, bps }),
        Outcome::Refused(why) => Err(why),
    })
}

/// Whether two servers' answers give the token the same implementation and
/// the same rules for this transfer.
pub(in crate::evm) fn token_agrees(
    chain: &Chain,
    first: &[Answer],
    second: &[Answer],
) -> Result<bool, NetError> {
    let span = |a: &[Answer]| -> Result<_, NetError> {
        let end = COMMON.checked_add(READS).ok_or(NetError::ReplyShape)?;
        let token = a.get(COMMON..end).ok_or(NetError::ReplyShape)?;
        super::super::nox_read::rules(chain, token)
    };
    Ok(span(first)? == span(second)?)
}

/// What a USDC transfer delivers, which is all of it, or why Circle's
/// contract would refuse it, from a transfer simulated as the sender.
pub(in crate::evm) fn usdc_arrival(
    simulated: &Answer,
    held: u128,
    amount: u128,
) -> Result<Arrival, &'static str> {
    if held < amount {
        return Err("The USDC balance does not cover that.");
    }
    match simulated {
        Answer::Result(_) => Ok(Arrival { arrives: amount, fee: 0, bps: 0 }),
        Answer::Reverted(data) => Err(super::super::usdc_revert::why(data)),
        Answer::Failed => Err("USDC refused this transfer when it was tried."),
    }
}
