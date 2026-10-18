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

//! Why a Shield action's button is grey, in the order its needs are checked.

use alloc::format;
use alloc::string::String;

use super::check::{address_ok, amount_ok, wait_ok};
use super::consts::OVERRIDE;
use crate::wallet::state::shield_ui::{ShieldUi, SHIELD_DEPOSIT, SHIELD_SEND};

/* Why the button on `screen` is grey: the first need not met, in the order
 * above, or nothing when it is lit. */
pub fn held_back(ui: &ShieldUi, screen: u8) -> Option<String> {
    if let Some(w) = ui.waiting {
        return Some(format!(
            "Waits for the shield service, which is {} now.",
            super::proving::doing(w.op)
        ));
    }
    if !ui.opened {
        return Some(String::from("Waits for the shield to open for this account."));
    }
    let send = screen == SHIELD_SEND;
    if !send && ui.size.is_none() {
        return Some(String::from("Pick a size above."));
    }
    if send && ui.to.is_empty() {
        return Some(String::from("Paste the payee's nox1 address into To (Ctrl+V)."));
    }
    if send && !address_ok(&ui.to) {
        return Some(format!(
            "To holds {} letters that are not a whole nox1 address, which is 2,009 letters. \
             Paste it again: a paste replaces what is there.",
            ui.to.len()
        ));
    }
    if send && !amount_ok(&ui.amount) {
        return Some(String::from("Type an amount above zero, with at most 18 decimals."));
    }
    if !wait_ok(ui) && screen != SHIELD_DEPOSIT {
        return Some(format!(
            "The notes are still waiting, as the meter shows. Type \"{OVERRIDE}\" to spend now."
        ));
    }
    None
}
