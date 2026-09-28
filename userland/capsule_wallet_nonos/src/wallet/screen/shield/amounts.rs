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

/*
 * Shielded balances as a person reads them: four places for ETH, two for
 * NOX, and a dash while no note has been found, since an empty note store
 * means nothing was scanned yet as often as it means nothing is held.
 */

use alloc::format;
use alloc::string::String;

use super::consts::{NOTE_ETH, NOTE_NOX};
use crate::wallet::nox::format_nox;
use crate::wallet::screen::amounts::UNREAD;
use crate::wallet::state::State;

const WEI: u128 = 1_000_000_000_000_000_000;

fn eth(wei: u128) -> String {
    format!("{}.{:04}", wei / WEI, (wei % WEI) / (WEI / 10_000))
}

fn nox(wei: u128) -> String {
    let mut out = [0u8; 64];
    let n = format_nox(wei, &mut out);
    String::from(core::str::from_utf8(&out[..n]).unwrap_or(UNREAD))
}

pub fn shielded_eth(state: &State) -> String {
    if state.notes.is_empty() {
        return String::from(UNREAD);
    }
    eth(state.notes.balance(NOTE_ETH))
}

pub fn shielded_nox(state: &State) -> String {
    if state.notes.is_empty() {
        return String::from(UNREAD);
    }
    nox(state.notes.balance(NOTE_NOX))
}
