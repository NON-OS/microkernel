//! The answers to `reads::calls`, read into what a quote needs, with every
//! pinned fact checked on the way: each pair's token order, and NOX's
//! implementation, pause and blacklist.

use super::quote::NoxState;
use super::reads::NOX_READS;
use super::route::Hop;
use crate::error::NetError;
use crate::evm::network::Chain;
use crate::evm::reply::Answer;

/// The market as one server saw it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct Market {
    /// Each hop's reserves, in then out.
    pub reserves: Vec<(u128, u128)>,
    /// The input token's balance and the router's allowance over it.
    pub held_in: Option<(u128, u128)>,
    pub nox: Option<NoxState>,
}

/// Why a market cannot be traded on, in one sentence.
pub(super) type Refusal = &'static str;

fn address(word: &[u8]) -> Option<[u8; 20]> {
    word.get(12..32)?.try_into().ok()
}

fn at(a: &[Answer], i: usize) -> Result<&Answer, NetError> {
    a.get(i).ok_or(NetError::ReplyShape)
}

/// The low 128 bits of the `k`th word of a call's return data.
pub(super) fn word(data: &[u8], k: usize) -> Result<u128, NetError> {
    let w = data.chunks_exact(32).nth(k).and_then(|w| w.get(16..)).ok_or(NetError::ReplyShape)?;
    Ok(u128::from_be_bytes(w.try_into().map_err(|_| NetError::ReplyShape)?))
}

pub(super) fn market(
    chain: &Chain,
    hops: &[Hop],
    token_in: bool,
    nox: bool,
    a: &[Answer],
) -> Result<Result<Market, Refusal>, NetError> {
    let mut i = 4usize;
    let mut reserves = Vec::new();
    for hop in hops {
        let r = at(a, i)?.data()?;
        let (r0, r1) = (word(&r, 0)?, word(&r, 1)?);
        let token0 = address(&at(a, i.saturating_add(1))?.data()?).ok_or(NetError::ReplyShape)?;
        if token0 != hop.pair.token0 {
            return Ok(Err("A pool is not the one this build was checked against."));
        }
        reserves.push(if hop.token_in == token0 { (r0, r1) } else { (r1, r0) });
        i = i.saturating_add(2);
    }
    let held_in = if token_in {
        let held = (at(a, i)?.word()?, at(a, i.saturating_add(1))?.word()?);
        i = i.saturating_add(2);
        Some(held)
    } else {
        None
    };
    let nox = if nox {
        let part = a.get(i..i.saturating_add(NOX_READS)).ok_or(NetError::ReplyShape)?;
        match super::parse_nox::state(chain, part)? {
            Ok(state) => Some(state),
            Err(why) => return Ok(Err(why)),
        }
    } else {
        None
    };
    Ok(Ok(Market { reserves, held_in, nox }))
}
