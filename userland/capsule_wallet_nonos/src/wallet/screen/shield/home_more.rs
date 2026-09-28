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
 * The quiet rows at the foot of Shield home: a proof in progress when
 * there is one, the history, and the network the pool is on.
 */

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::shield_ui::{SHIELD_HISTORY, SHIELD_NETWORK, SHIELD_PROVING};
use crate::wallet::state::State;

pub fn more(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let mut rows: Vec<(&str, bool, u8)> =
        Vec::from([("History", true, SHIELD_HISTORY), ("Sepolia network", true, SHIELD_NETWORK)]);
    if state.shield_ui.job.is_some() {
        rows.insert(0, ("Proof in progress", true, SHIELD_PROVING));
    }
    let shown: Vec<(&str, bool)> = rows.iter().map(|(t, e, _)| (*t, *e)).collect();
    let mut at = [Rect::default(); 3];
    let h = quiet_group(fb, c.x, y, c.w, &shown, &mut at);
    for (i, (_, _, screen)) in rows.iter().enumerate() {
        hits::put(Press::Go(*screen), at[i]);
    }
    h
}
