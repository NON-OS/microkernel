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
//! The selected listing: what it is, who stands behind it, and whether this
//! machine will run it.

use nonos_app_skeleton::PaintBuffer;

use crate::store::state::State;
use crate::store::theme::{ACCENT, FOREGROUND, MUTED, PANE_BG, TITLE};
use super::hex::short;
use super::metrics::{BODY_PX, DETAIL_PAD, SMALL_PX, TITLE_PX};
use super::text;
use super::wrap::wrap;

pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_rect(x, y, w, h, PANE_BG);
    let left = x + DETAIL_PAD;
    let room = w.saturating_sub(DETAIL_PAD * 2);
    let Some(listing) = state.current() else {
        text::line(fb, left, y as i32 + DETAIL_PAD as i32, b"Nothing selected", MUTED, BODY_PX);
        return;
    };
    let mut top = y as i32 + DETAIL_PAD as i32;
    text::line(fb, left, top, &listing.name, TITLE, TITLE_PX);
    top += 32;

    if let Some(d) = &state.detail {
        text::line(fb, left, top, &d.publisher, ACCENT, SMALL_PX);
        top += 26;
        for line in wrap(&d.description, room, SMALL_PX).iter().take(4) {
            text::line(fb, left, top, line, FOREGROUND, SMALL_PX);
            top += 20;
        }
        top += 10;
    }

    top = super::standing::paint(fb, state, left, top);
    text::line(fb, left, top, b"measurement", MUTED, SMALL_PX);
    top += 20;
    text::line(fb, left, top, &short(&listing.measurement), MUTED, SMALL_PX);
}
