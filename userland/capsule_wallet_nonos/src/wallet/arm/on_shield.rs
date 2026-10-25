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

//! The Shield's guarded screens (moves.rs): a review, Proving with Take
//! back, and home with a landed row offering its approval or deposits.

use super::moves::Shown;
use crate::wallet::screen::shield::landed_move::{landed, ROW_MOVE};
use crate::wallet::screen::shield::proving::{buttons, TAKE_BACK};
use crate::wallet::shield::fresh_move_act::{act, moves, Act};
use crate::wallet::state::shield_ui::{SHIELD_HOME, SHIELD_PROVING, SHIELD_REVIEW};
use crate::wallet::state::State;

/// As the Shield's footer and landed rows read (screen/shield/click.rs).
pub fn shield(state: &State) -> Option<Shown> {
    let ui = &state.shield_ui;
    match ui.screen {
        SHIELD_REVIEW => Some(Shown::ShieldReview),
        SHIELD_PROVING => {
            let at = buttons(ui).iter().position(|b| b.1 == TAKE_BACK)?;
            Some(Shown::TakeBack(at as u8))
        }
        SHIELD_HOME => {
            let (now, moving) = (crate::wallet::clock::now_s(), state.meter.landed.moving.as_ref());
            (0u8..).zip(landed(state)).find_map(|(i, (address, held))| {
                let (eth, _) = moves(held)?;
                let m = moving.filter(|m| m.address == address && m.eth == eth);
                let confirm = matches!(act(m, now), Act::Confirm { .. });
                confirm.then_some(Shown::Landed(ROW_MOVE + i))
            })
        }
        _ => None,
    }
}
