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

//! The strip along the bottom: what the keys do, and what the last
//! install request was told.

use nonos_app_skeleton::PaintBuffer;

use crate::store::state::State;
use crate::store::theme::{ACCENT, MUTED, RULE, STATUS_BG};

use super::metrics::{SMALL_PX, STATUS_H, STATUS_PAD_X};
use super::text;

pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let y = state.fb_h.saturating_sub(STATUS_H);
    fb.fill_rect(0, y, state.fb_w, STATUS_H, STATUS_BG);
    fb.fill_rect(0, y, state.fb_w, 1, RULE);
    let top = text::top_of(y as i32, STATUS_H, SMALL_PX);
    let keys: &[u8] = b"up/down select    Enter install    o open    r refresh    Esc close";
    text::line(fb, STATUS_PAD_X, top, keys, MUTED, SMALL_PX);
    // The answer to the last request sits opposite the keys.
    let right = state.fb_w.saturating_sub(STATUS_PAD_X);
    if let Some(asked) = state.asked {
        text::right(fb, right, top, asked.label(), ACCENT, SMALL_PX);
    }
}
