//! The token's state, from the answers to `nox::reads`.
//!
//! The proxy's implementation slot is read first: the rules in `nox_rules`
//! are those of the implementations this build lists, and a token upgraded to
//! anything else stops NOX sends until the app is checked against it.

use super::network::Chain;
use super::nox_rules::Rules;
use super::reply::Answer;
use crate::error::NetError;

/// The token's state from the answers to `reads`, or none when its
/// implementation is not one this build was checked against. The last
/// answer, the simulation, is left to the caller.
pub(super) fn rules(chain: &Chain, a: &[Answer]) -> Result<Option<Rules>, NetError> {
    let word =
        |i: usize| -> Result<Vec<u8>, NetError> { a.get(i).ok_or(NetError::ReplyShape)?.data() };
    let flag = |i: usize| -> Result<bool, NetError> { Ok(word(i)?.last() == Some(&1)) };
    let slot = word(0)?;
    let implementation = slot.get(12..32).ok_or(NetError::ReplyShape)?;
    if !chain.nox_implementations.iter().any(|known| known.as_slice() == implementation) {
        return Ok(None);
    }
    let fees = word(4)?;
    let bps = |k: usize| -> Result<u16, NetError> {
        let at = k.checked_mul(32).and_then(|x| x.checked_add(30)).ok_or(NetError::ReplyShape)?;
        let pair = fees.get(at..at.saturating_add(2)).ok_or(NetError::ReplyShape)?;
        Ok(u16::from_be_bytes([
            pair.first().copied().unwrap_or(0),
            pair.get(1).copied().unwrap_or(0),
        ]))
    };
    Ok(Some(Rules {
        paused: flag(1)?,
        blocked: flag(2)? || flag(3)?,
        fees: [bps(0)?, bps(1)?, bps(2)?],
        pair_from: flag(5)?,
        pair_to: flag(6)?,
        exempt: flag(7)? || flag(8)?,
    }))
}
