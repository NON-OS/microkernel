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
//! NOX amounts and the typed figure as the staking screen says them.

use alloc::format;
use alloc::string::String;

use crate::wallet::nox::{format_nox, held_wei};
use crate::wallet::state::State;

fn nox_text(wei: u128) -> String {
    let mut b = [0u8; 64];
    let text = format_nox(wei, &mut b).and_then(|n| core::str::from_utf8(&b[..n]).ok());
    String::from(text.unwrap_or("?"))
}

/// A NOX amount in wei, as the review says it.
pub fn nox_amount(wei: u128) -> String {
    nox_text(wei)
}

pub(super) fn word(ready: bool, w: &[u8; 32]) -> String {
    match held_wei(ready, w) {
        Some(v) => format!("{} NOX", nox_text(v)),
        None => String::from(super::super::amounts::UNREAD),
    }
}

/// The figure in the stake field as it was typed.
pub fn typed(state: &State) -> String {
    if state.stake.digits == 0 && state.stake.amount == 0 {
        return String::new();
    }
    // A figure set whole (Use all) is held in wei, not as typed digits.
    if state.stake.digits == 0 {
        return nox_text(state.stake.amount);
    }
    let digits = format!("{}", state.stake.amount);
    if !state.stake.point {
        return digits;
    }
    let places = state.stake.places as usize;
    let padded = format!("{:0>width$}", digits, width = places + 1);
    let (whole, frac) = padded.split_at(padded.len() - places);
    format!("{whole}.{frac}")
}

/// What the button signs next, said plainly.
pub fn action(state: &State) -> String {
    if state.stake.unstake == 1 {
        return format!("Review closing position {}", state.stake.position);
    }
    let wei = crate::wallet::event::stake_wei(state);
    let verb = if state.stake.step == 0 { "Review approval of" } else { "Review stake of" };
    format!("{verb} {} NOX", nox_text(wei))
}
