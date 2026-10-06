// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! 07  STAKE NOX, on Ethereum mainnet: lock NOX into the staking contract
//! for a share of its emission, or close a position. Staking is two
//! transactions, the allowance and then the stake, and the button says
//! which one it signs next. Sepolia has no staking contract, so there the
//! screen says so and signs nothing.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use super::page::{edges, lead};
use crate::wallet::chain;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::{GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::nox::{format_apr, format_nox, held_wei, lock_days, LOCK_TERMS};
use crate::wallet::screen::shield::parts::{chips, field};
use crate::wallet::state::State;

pub mod click;
mod review;

const LEAD: &str = "Staked NOX earns a share of the emission. A lock term weighs the stake \
     more and keeps it in until the term ends.";
/* Positions offered to close at once; the rest are said, not hidden. */
const SHOWN: u64 = 8;
const NO_STAKING: &str = "Staking runs on Ethereum mainnet. Switch networks in Settings to \
     stake.";

fn nox_text(wei: u128) -> String {
    let mut b = [0u8; 64];
    let text = format_nox(wei, &mut b).and_then(|n| core::str::from_utf8(&b[..n]).ok());
    String::from(text.unwrap_or("?"))
}

/// A NOX amount in wei, as the review says it.
pub fn nox_amount(wei: u128) -> String {
    nox_text(wei)
}

fn word(ready: bool, w: &[u8; 32]) -> String {
    match held_wei(ready, w) {
        Some(v) => format!("{} NOX", nox_text(v)),
        None => String::from(super::amounts::UNREAD),
    }
}

/// The figure in the stake field as it was typed.
pub fn typed(state: &State) -> String {
    if state.stake_digits == 0 && state.stake_amount == 0 {
        return String::new();
    }
    // A figure set whole (Use all) is held in wei, not as typed digits.
    if state.stake_digits == 0 {
        return nox_text(state.stake_amount);
    }
    let digits = format!("{}", state.stake_amount);
    if !state.stake_point {
        return digits;
    }
    let places = state.stake_places as usize;
    let padded = format!("{:0>width$}", digits, width = places + 1);
    let (whole, frac) = padded.split_at(padded.len() - places);
    format!("{whole}.{frac}")
}

/// What the button signs next, said plainly.
pub fn action(state: &State) -> String {
    if state.stake_unstake == 1 {
        return format!("Review closing position {}", state.stake_position);
    }
    let wei = crate::wallet::event::stake_wei(state);
    let verb = if state.stake_step == 0 { "Review approval of" } else { "Review stake of" };
    format!("{verb} {} NOX", nox_text(wei))
}

pub fn show(state: &State, fb: &mut PaintBuffer) {
    if let Some(d) = &state.stake_draft {
        return review::review(state, fb, d);
    }
    hits::clear();
    let status = super::status::parts(state);
    let mainnet = chain::current().staking.is_some();
    /* While a transaction goes out the button says so and takes no press. */
    let working = crate::wallet::act::working(state);
    let label = working.map_or_else(|| action(state), String::from);
    let ok = mainnet
        && working.is_none()
        && crate::wallet::event::stake_refusal(state).is_none()
        && crate::wallet::send::held_back(state).is_none();
    let footer =
        [(label.as_str(), Weight::Primary, ok), ("Use all NOX", Weight::Secondary, mainnet)];
    let shown = if state.stake_unstake == 1 { &footer[..1] } else { &footer[..] };
    let spec = FrameSpec {
        number: "07",
        title: "Stake NOX",
        back: true,
        backdrop: Some(Backdrop::Deposit),
        failure: state.failure,
        footer: shown,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, if mainnet { LEAD } else { NO_STAKING });
    y += chips(fb, c, y, &["Stake", "Unstake"], Some(state.stake_unstake), Press::Tab) + GAP;
    if state.stake_unstake == 0 {
        let (at, h) = field(fb, c, y, "AMOUNT, NOX", &typed(state), "0.0", true);
        hits::put(Press::Field(0), at);
        y += h + GAP / 2;
        let terms: Vec<String> = LOCK_TERMS
            .iter()
            .map(
                |(s, _)| {
                    if *s == 0 {
                        String::from("no lock")
                    } else {
                        format!("{} d", lock_days(*s))
                    }
                },
            )
            .collect();
        let refs: Vec<&str> = terms.iter().map(String::as_str).collect();
        y += chips(fb, c, y, &refs, Some(state.stake_lock), Press::Term) + GAP;
    } else {
        let n = if state.nox.positions_ready { state.nox.positions.min(SHOWN) } else { 0 };
        let names: Vec<String> = (0..n).map(|i| format!("#{i}")).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        if !state.nox.positions_ready {
            y += fact(fb, c.x, y, c.w, "positions", super::amounts::UNREAD);
        } else if refs.is_empty() {
            y += fact(fb, c.x, y, c.w, "positions", "none open");
        } else {
            y += chips(fb, c, y, &refs, Some(state.stake_position as u8), Press::Pick) + GAP;
            if state.nox.positions > SHOWN {
                let more = format!("the first {SHOWN} of {}", state.nox.positions);
                y += fact(fb, c.x, y, c.w, "shown", &more);
            }
        }
    }
    /* A button that takes no press says why. */
    let why = crate::wallet::event::stake_refusal(state)
        .and_then(|w| core::str::from_utf8(w).ok())
        .or_else(|| crate::wallet::send::held_back(state));
    if let (true, None, Some(why)) = (mainnet, working, why) {
        let (x, w) = (c.x as i32, c.w as i32);
        y += wrapped(fb, x, y as i32, w, Role::Lead, why, TEXT_3) as u32 + GAP / 2;
    }
    y += fact(
        fb,
        c.x,
        y,
        c.w,
        "this account holds",
        &word(state.nox.balance_ready, &state.nox.balance_wei),
    );
    y += fact(
        fb,
        c.x,
        y,
        c.w,
        "rewards to claim",
        &word(state.nox.claimable_ready, &state.nox.claimable_wei),
    );
    let positions = if state.nox.positions_ready {
        format!("{}", state.nox.positions)
    } else {
        String::from(super::amounts::UNREAD)
    };
    y += fact(fb, c.x, y, c.w, "open positions", &positions);
    let apr = if state.nox.apr_ready {
        let mut b = [0u8; 24];
        let n = format_apr(state.nox.apr_bps, &mut b);
        String::from(core::str::from_utf8(&b[..n]).unwrap_or("?"))
    } else {
        String::from(super::amounts::UNREAD)
    };
    y += fact(fb, c.x, y, c.w, "rate now", &apr);
    y += fact(
        fb,
        c.x,
        y,
        c.w,
        "total staked",
        &word(state.nox.stats_ready, &state.nox.total_staked_wei),
    );
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ok, mainnet && state.stake_unstake == 0]);
}
