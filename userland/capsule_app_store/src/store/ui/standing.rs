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

//! Where the selected listing stands with this machine.

use nonos_app_skeleton::PaintBuffer;

use crate::store::state::State;
use crate::store::theme::{DANGER, MUTED};

use super::gates;
use super::headline::headline;
use super::metrics::{BODY_PX, SMALL_PX};
use super::text;
use super::wrap::wrap;

/// At most this many lines of the sentence: a reason a person must act on
/// is read whole, not cut at the pane's edge.
const SENTENCE_LINES: usize = 2;

/// Paints and returns the y to carry on from. `room` is the pane's width
/// for text.
pub fn paint(fb: &mut PaintBuffer, state: &State, left: u32, room: u32, mut top: i32) -> i32 {
    let Some(listing) = state.current() else { return top };
    match listing.known.readiness {
        Some(r) => {
            let (line, tone) = headline(state, listing, &r);
            let line = &line[..];
            for part in wrap(line, room, BODY_PX).iter().take(SENTENCE_LINES) {
                text::line(fb, left, top, part, tone, BODY_PX);
                top += 22;
            }
            top += 8;
            top = super::tier_path::paint(fb, state, listing, left, room, top);
            // Where the gates end, not a guess at their height: a guess once
            // put the measurement on top of the last gate.
            top = gates::paint(fb, left, top, &r) + 14;
        }
        /*
         * Asked and not answered, or the market went quiet before this one
         * was asked: say so, rather than "checking" for ever.
         */
        None if listing.known.judged || state.port == 0 => {
            text::line(
                fb,
                left,
                top,
                b"the market did not say; press r to ask again",
                DANGER,
                SMALL_PX,
            );
            top += 26;
        }
        None => {
            text::line(fb, left, top, b"checking", MUTED, SMALL_PX);
            top += 26;
        }
    }
    top
}
