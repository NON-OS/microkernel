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

//! One of the four things a holder does from home: a 56 pixel cyan circle
//! with its sign in ink and a word under it. Disabled, the circle is an
//! outline and the sign and word are text-3.

use nonos_app_skeleton::PaintBuffer;

use super::super::rect::Rect;
use super::super::roles::Role;
use super::super::symbol::{symbol, Symbol};
use super::super::text::{draw_in, line, width};
use super::super::tokens::{CYAN, INK, LINE_2, ROUND, TEXT, TEXT_3};

/// The sign sits 8 above its word.
const UNDER: u32 = 8;

pub fn round_height() -> u32 {
    ROUND + UNDER + line(Role::ActionLabel) as u32
}

/// Draw centred in `at`, which the caller keeps for the click.
pub fn round_action(fb: &mut PaintBuffer, at: Rect, title: &str, sign: Symbol, enabled: bool) {
    let cx = at.x + at.w / 2;
    let cy = at.y + ROUND / 2;
    if enabled {
        fb.circle(cx, cy, ROUND / 2, CYAN);
    } else {
        fb.ring(cx, cy, ROUND / 2, 1, LINE_2);
    }
    symbol(fb, cx as i32, cy as i32, sign, if enabled { INK } else { TEXT_3 });
    let tx = cx as i32 - width(Role::ActionLabel, title) / 2;
    let ink = if enabled { TEXT } else { TEXT_3 };
    draw_in(fb, tx, (at.y + ROUND + UNDER) as i32, Role::ActionLabel, title, ink);
}
