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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Rect};

use super::bytes::as_str;
use super::hex::short;
use super::metrics::metrics;
use crate::store::state::State;

/// How far each part of the pane drops the next below it, in logical pixels:
/// the name, the publisher, a version or description line, the air before
/// and after the description, and the release note.
const TITLE_DROP: u32 = 32;
const PUBLISHER_DROP: u32 = 22;
const LINE: u32 = 20;
const BEFORE_DESCRIPTION: u32 = 6;
const AFTER_DESCRIPTION: u32 = 10;
const NOTE_DROP: u32 = 24;

pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    let (r, m, s) = (roles(), metrics(), scale());
    let drop = |logical: u32| s.px(logical) as i32;
    Card::new().draw(fb, Rect { x, y, w, h });
    let left = x + m.detail_pad;
    let room = w.saturating_sub(m.detail_pad * 2);
    let mut top = y as i32 + m.detail_pad as i32;
    let Some(listing) = state.current() else {
        text::left(fb, left, top, "Nothing selected", r.muted, m.body_px);
        return;
    };
    text::left(fb, left, top, as_str(&listing.name), r.text, m.title_px);
    top += drop(TITLE_DROP);

    if let Some(d) = &listing.known.detail {
        text::left(fb, left, top, as_str(&d.publisher), r.accent, m.small_px);
        top += drop(PUBLISHER_DROP);
        /*
         * Which version, before anything else. Where the bytes come from is
         * provenance the install checks, not a label.
         */
        if let Some(rel) = &listing.known.release {
            let mut line = b"Version ".to_vec();
            line.extend_from_slice(rel.short_version());
            text::left(fb, left, top, as_str(&line), r.muted, m.small_px);
            top += drop(LINE);
        }
        top += drop(BEFORE_DESCRIPTION);
        let description = as_str(&d.description);
        for (start, end) in text::wrap(description, room, m.small_px).into_iter().take(4) {
            text::left(fb, left, top, &description[start..end], r.text_2, m.small_px);
            top += drop(LINE);
        }
        top += drop(AFTER_DESCRIPTION);
    }

    top = super::standing::paint(fb, state, left, room, top);
    if let Some(rel) = listing.known.release.as_ref().filter(|rel| !rel.note.is_empty()) {
        text::left(fb, left, top, as_str(&rel.note), r.muted, m.small_px);
        top += drop(NOTE_DROP);
    }
    /* One line, so it stays inside the pane under a failure sentence and a note. */
    let mut line = b"measurement  ".to_vec();
    line.extend_from_slice(&short(&listing.measurement));
    text::left(fb, left, top, as_str(&line), r.muted, m.small_px);
}
