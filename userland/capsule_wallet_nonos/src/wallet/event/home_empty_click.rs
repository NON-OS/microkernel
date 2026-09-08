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

//! The two buttons on the empty account card.

use nonos_app_skeleton::EventOutcome;

use super::on_pointer::hit;
use crate::wallet::paint::home_geom::{LEFT, TOP};
use crate::wallet::paint::paint_account_card::CARD_H;
use crate::wallet::state::State;

// Mirrors paint_account_empty: both buttons sit on the card's bottom padding.
const PAD: u32 = 24;
const BTN_W: u32 = 190;
const BTN_H: u32 = 42;

/// `None` when there is an account, so the caller falls through to the quick
/// actions that occupy the same screen once the card is a balance again.
pub(super) fn empty_state(state: &mut State, x: u32, y: u32) -> Option<EventOutcome> {
    if state.address_ready {
        return None;
    }
    let bx = LEFT + PAD;
    let by = TOP + CARD_H.saturating_sub(PAD + BTN_H);
    if hit(x, y, bx, by, BTN_W, BTN_H) {
        return Some(super::generate::generate(state));
    }
    if hit(x, y, bx + BTN_W + 14, by, BTN_W, BTN_H) {
        return Some(super::import::toggle_import(state));
    }
    None
}
