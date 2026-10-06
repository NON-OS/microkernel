//! The checks a swap passes before anything is built: a sane order, two
//! servers that agree about the pools, and a quote whose price impact and
//! minimum a person would accept.

use super::math::mul_div;
use super::order::Swap;
use super::parse::Market;
use super::quote::quote;
use super::route::Hop;

/// The most slippage a swap may allow: 5%.
const MAX_SLIPPAGE_BPS: u16 = 500;
/// The most a swap may move a pool's price: 15%.
const MAX_IMPACT_BPS: u128 = 1_500;

pub(super) struct Plan {
    pub expected: u128,
    pub minimum: u128,
    pub token_fee: u128,
    pub impact_bps: u128,
    pub approval: bool,
}

pub(super) fn before_reading(swap: &Swap) -> Option<&'static str> {
    if swap.amount == 0 {
        return Some("Enter an amount above zero.");
    }
    if swap.slippage_bps > MAX_SLIPPAGE_BPS {
        return Some("Allow at most 5% slippage.");
    }
    None
}

/// The same NOX state from both servers, and reserves within 1% of each
/// other: one block apart is fine, a different pool is not.
pub(super) fn agree(a: &Market, b: &Market) -> bool {
    let close = |x: u128, y: u128| {
        let (lo, hi) = if x < y { (x, y) } else { (y, x) };
        mul_div(hi.saturating_sub(lo), 100, hi.max(1)).is_some_and(|pct| pct < 1)
    };
    a.nox == b.nox
        && a.reserves.len() == b.reserves.len()
        && a.reserves.iter().zip(&b.reserves).all(|(p, q)| close(p.0, q.0) && close(p.1, q.1))
}

pub(super) fn plan(hops: &[Hop], market: &Market, swap: &Swap) -> Result<Plan, &'static str> {
    if let Some((held, _)) = market.held_in {
        if held < swap.amount {
            return Err("The balance does not cover that.");
        }
    }
    let q = quote(hops, &market.reserves, swap.amount, market.nox)
        .ok_or("The pools cannot fill that.")?;
    let raw =
        quote(hops, &market.reserves, swap.amount, None).ok_or("The pools cannot fill that.")?;
    let mut ideal = Some(swap.amount);
    for &(r_in, r_out) in &market.reserves {
        ideal = ideal.and_then(|x| mul_div(x, r_out, r_in));
    }
    let ideal = ideal.ok_or("The pools cannot fill that.")?;
    let impact_bps = mul_div(ideal.saturating_sub(raw.out), 10_000, ideal.max(1)).unwrap_or(10_000);
    if impact_bps > MAX_IMPACT_BPS {
        return Err("That moves the price by more than 15%. Swap less.");
    }
    let keep = 10_000u128.saturating_sub(u128::from(swap.slippage_bps));
    let minimum = mul_div(q.out, keep, 10_000).ok_or("The pools cannot fill that.")?;
    if minimum == 0 {
        return Err("That is too small to swap.");
    }
    let approval = market.held_in.is_some_and(|(_, allowed)| allowed < swap.amount);
    Ok(Plan { expected: q.out, minimum, token_fee: q.token_fee, impact_bps, approval })
}
