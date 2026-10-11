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

//! The service's own state: the receive address and the shielded holdings.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use shield_wire::*;

use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

/* The service's own state: the receive address and the shielded holdings. */
pub fn state(state: &mut State, body: &str) {
    let ui = &mut state.shield_ui;
    if let Some(a) = field(body, "address") {
        ui.nox1 = Some(String::from(a));
    }
    /* Read again from a service that already held the store. */
    if let Some(kept) = field(body, "kept") {
        ui.live = kept == "memory";
    }
    if let Some(history) = super::reply::entries(body) {
        ui.history = history;
        ui.history_cut = field(body, "history_cut").and_then(|n| n.parse().ok()).unwrap_or(0);
    }
    /* What the store would not give is said, and what was shown stays. */
    let unread = [("balances_why", "balances"), ("history_why", "history")];
    for (name, what) in unread {
        if let Some(why) = field(body, name) {
            ui.failure = Some(format!("The shielded {what} could not be read: {why}"));
        }
    }
    if field(body, "balances") != Some("1") {
        return;
    }
    let mut held = [None, None];
    for line in fields(body, "balance") {
        let parts: Vec<&str> = line.split(' ').collect();
        let [coin, spendable, pending, _notes] = parts[..] else { continue };
        let slot = match coin {
            "ETH" => 0,
            "NOX" => 1,
            _ => continue,
        };
        held[slot] =
            Some(Held { spendable: String::from(spendable), pending: String::from(pending) });
    }
    ui.held = held;
}
