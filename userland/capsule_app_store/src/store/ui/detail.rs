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

use super::hex::short;
use super::metrics::{BODY_PX, DETAIL_PAD, SMALL_PX, TITLE_PX};
use super::text;
use super::wrap::wrap;
use crate::store::state::State;
use crate::store::theme::{ACCENT, FOREGROUND, MUTED, PANE_BG, TITLE};

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
        top += 22;
        // Which version, before anything else. Where the bytes come from is
        // provenance the install checks, not a label.
        if let Some(r) = &state.release {
            let mut line = b"Version ".to_vec();
            line.extend_from_slice(r.version.rsplit(|b| *b == b'@').next().unwrap_or(&r.version));
            text::line(fb, left, top, &line, MUTED, SMALL_PX);
            top += 20;
        }
        top += 6;
        for line in wrap(&d.description, room, SMALL_PX).iter().take(4) {
            text::line(fb, left, top, line, FOREGROUND, SMALL_PX);
            top += 20;
        }
        top += 10;
    }

    top = super::standing::paint(fb, state, left, top);
    if let Some(r) = state.release.as_ref().filter(|r| !r.note.is_empty()) {
        text::line(fb, left, top, &r.note, MUTED, SMALL_PX);
        top += 24;
    }
    // One line, so it stays inside the pane under a failure sentence and a note.
    let mut line = b"measurement  ".to_vec();
    line.extend_from_slice(&short(&listing.measurement));
    text::line(fb, left, top, &line, MUTED, SMALL_PX);
}
