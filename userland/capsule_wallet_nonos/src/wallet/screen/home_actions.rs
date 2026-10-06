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

//! The things a holder does from home, a row of round actions. Send waits
//! for the balance, since there is nothing to send until it is read.

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::parts::round::{round_action, round_height};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::symbol::Symbol;
use crate::wallet::state::State;

const ACTIONS: [(&str, Symbol, Press); 4] = [
    ("Send", Symbol::ArrowUp, Press::Send),
    ("Receive", Symbol::ArrowDown, Press::Receive),
    ("Swap", Symbol::ArrowLeftRight, Press::Swap),
    ("Shield", Symbol::LockShield, Press::Shield),
];

/* Swap is not offered in this build: no liquidity pool is wired to quote
 * or sign against (`pool::active` answers NotWired), so its screen could
 * only say so. It returns once a pool is. */
const SWAP_OFFERED: bool = false;

pub fn actions(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let shown: alloc::vec::Vec<&(&str, Symbol, Press)> =
        ACTIONS.iter().filter(|(_, _, p)| *p != Press::Swap || SWAP_OFFERED).collect();
    let cell = c.w / shown.len() as u32;
    for (i, (title, sign, press)) in shown.into_iter().enumerate() {
        let enabled = *press != Press::Send || state.balance_ready;
        let at = Rect::new(c.x + cell * i as u32, y, cell, round_height());
        round_action(fb, at, title, *sign, enabled);
        if enabled {
            hits::put(*press, at);
        }
    }
    round_height()
}
