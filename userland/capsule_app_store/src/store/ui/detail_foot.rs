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

//! The foot of the detail pane: the operator's note and the measurement.
//! The note wraps inside the pane, as the description does, and no line is
//! drawn past the pane's foot, so a long note is never cut off at the edge.

use nonos_app_skeleton::PaintBuffer;

use super::hex::short;
use super::metrics::SMALL_PX;
use super::text;
use super::wrap::wrap;
use crate::store::listing::Listing;
use crate::store::theme::MUTED;

/// The most lines the note takes.
const NOTE_LINES: usize = 3;
const LINE: i32 = 20;

pub fn paint(
    fb: &mut PaintBuffer,
    listing: &Listing,
    left: u32,
    room: u32,
    mut top: i32,
    bottom: i32,
) {
    let foot = bottom - LINE;
    if let Some(r) = listing.known.release.as_ref().filter(|r| !r.note.is_empty()) {
        for part in wrap(&r.note, room, SMALL_PX).iter().take(NOTE_LINES) {
            if top > foot {
                return;
            }
            text::line(fb, left, top, part, MUTED, SMALL_PX);
            top += LINE;
        }
        top += 4;
    }
    let mut line = b"measurement  ".to_vec();
    line.extend_from_slice(&short(&listing.measurement));
    if top <= foot {
        text::line(fb, left, top, &line, MUTED, SMALL_PX);
    }
}
