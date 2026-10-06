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

//! Balances as a person reads them, always from the wallet's own figures:
//! four places for ETH, two for NOX, and a dash until the figure is read,
//! as the phones show a coin not yet heard from.

use alloc::format;
use alloc::string::String;

use crate::wallet::nox::{format_nox, held_wei};
use crate::wallet::state::State;

const WEI: u128 = 1_000_000_000_000_000_000;
pub const UNREAD: &str = "\u{2013}";

/// The low 128 bits of a 256-bit word: every real balance, where 64 bits
/// stopped at 18.4 ETH.
fn low_u128(v: &[u8; 32]) -> u128 {
    let mut b = [0u8; 16];
    b.copy_from_slice(&v[16..]);
    u128::from_be_bytes(b)
}

pub fn eth(state: &State) -> String {
    if !state.balance_ready {
        return String::from(UNREAD);
    }
    let wei = low_u128(&state.balance_wei);
    format!("{}.{:04}", wei / WEI, (wei % WEI) / (WEI / 10_000))
}

pub fn nox(state: &State) -> String {
    match held_wei(state.nox.balance_ready, &state.nox.balance_wei) {
        Some(wei) => {
            let mut out = [0u8; 64];
            let text = format_nox(wei, &mut out).and_then(|n| core::str::from_utf8(&out[..n]).ok());
            String::from(text.unwrap_or(UNREAD))
        }
        None => String::from(UNREAD),
    }
}

/// Two places, as a dollar amount is read.
pub fn usdc(state: &State) -> String {
    if !state.usdc_ready {
        return String::from(UNREAD);
    }
    let units = low_u128(&state.usdc_units);
    format!("{}.{:02}", units / 1_000_000, (units % 1_000_000) / 10_000)
}
