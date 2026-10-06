//! What a route pays out, hop by hop, with NOX's own rules where NOX moves.
//! Selling NOX, the sell fee comes off what reaches the pair, and past the
//! threshold the token first sells a chunk of its fees into the same pair, per
//! `NOXTokenV3_1._update`, which moves the price before the seller's tokens land.
//! Buying NOX, the buy fee comes off what the pair sends. An exempt account pays neither.

use super::math::{amount_out, mul_div};
use super::route::Hop;
use nox_verified::fee::split_fee;

/// The NOX state a quote depends on, read from the chain.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NoxState {
    pub token: [u8; 20],
    pub buy_bps: u16,
    pub sell_bps: u16,
    pub burn_share_bps: u16,
    pub exempt: bool,
    /// The fee sale, when it is switched on: threshold, chunk, fees held.
    pub fee_sale: Option<(u128, u128, u128)>,
}

/// A route's outcome: what arrives, and the token fee taken on the way.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Quote {
    pub out: u128,
    pub token_fee: u128,
}

/// Walk `hops` with `input`, each hop's reserves given in and out.
pub fn quote(
    hops: &[Hop],
    reserves: &[(u128, u128)],
    input: u128,
    nox: Option<NoxState>,
) -> Option<Quote> {
    let mut amount = input;
    let mut token_fee = 0u128;
    for (hop, &(mut r_in, mut r_out)) in hops.iter().zip(reserves) {
        let selling = nox.filter(|n| n.token == hop.token_in);
        if let Some(n) = selling {
            let (into, fee) = if n.exempt { (amount, 0) } else { split_fee(amount, n.sell_bps)? };
            if let Some((threshold, chunk, held)) = n.fee_sale {
                let kept =
                    mul_div(fee, 10_000u128.checked_sub(u128::from(n.burn_share_bps))?, 10_000)?;
                let after = held.checked_add(kept)?;
                if after >= threshold {
                    let sold = if chunk == 0 { after } else { after.min(chunk) };
                    let paid = amount_out(sold, r_in, r_out)?;
                    r_in = r_in.checked_add(sold)?;
                    r_out = r_out.checked_sub(paid)?;
                }
            }
            token_fee = token_fee.checked_add(fee)?;
            amount = into;
        }
        amount = amount_out(amount, r_in, r_out)?;
        if let Some(n) = nox.filter(|n| n.token == hop.token_out && !n.exempt) {
            let (arrives, fee) = split_fee(amount, n.buy_bps)?;
            token_fee = token_fee.checked_add(fee)?;
            amount = arrives;
        }
    }
    Some(Quote { out: amount, token_fee })
}

#[cfg(test)]
#[path = "quote_test.rs"]
mod quote_test;
