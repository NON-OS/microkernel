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

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_battery_state;

use crate::about::data::battery::rows;

use super::super::card::{self, titled};
use super::super::kv::{kv, ROW_H};
use super::super::metrics::CARD_PAD;

pub const HEIGHT: u32 = card::OVERHEAD + ROW_H * 3;

// The battery as the bar reads it, from the same call: the charge, whether
// firmware says it is charging, and the AC adapter. A row with nothing to
// say (a desktop's state) is left out rather than drawn as a guess.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32) {
    let top = titled(fb, x, y, w, HEIGHT, b"Power");
    let mut pct = [0u8; 4];
    let [charge, state, ac] = rows(mk_battery_state(), &mut pct);
    let all: [(&[u8], &[u8]); 3] = [(b"Battery", charge), (b"State", state), (b"AC", ac)];
    for (i, (label, value)) in all.into_iter().filter(|r| !r.1.is_empty()).enumerate() {
        let row_y = top + (i as u32 * ROW_H) as i32;
        kv(fb, x + CARD_PAD, row_y, card::inner(w), label, value, false);
    }
}
