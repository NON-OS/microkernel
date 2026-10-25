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

//! The proof, under the listing's facts: the question with its two keys,
//! then while it is made the stage line in their place, then what came of
//! it, kept until the listing is left.

use nonos_app_skeleton::PaintBuffer;

use super::metrics::{BODY_PX, SMALL_PX};
use super::text;
use super::wrap::wrap;
use crate::store::proof::proof_said::{again, said, STOPPED};
use crate::store::proof::proof_seq::Phase;
use crate::store::proof::proof_words::{question, working, CHECKED, NOT_NOW, PROVE_IT, READY};
use crate::store::state::State;
use crate::store::theme::{ACCENT, BUTTON_BG, BUTTON_FG, DANGER, FOREGROUND, MUTED, OK};

const KEY_W: u32 = 112;
const KEY_H: u32 = 26;

/// Paints and returns the y to carry on from.
pub fn paint(fb: &mut PaintBuffer, state: &State, left: u32, room: u32, top: i32) -> i32 {
    let Some(l) = state.current() else { return top };
    let p = &state.proofs;
    if p.asking.as_deref() == Some(&l.id[..]) {
        let mut top = lines(fb, &question(&l.name), FOREGROUND, BODY_PX, (left, room, top));
        top = lines(fb, CHECKED, MUTED, SMALL_PX, (left, room, top)) + 4;
        // The primary key filled, as Install is; the quiet one only lettered.
        let y = text::top_of(top, KEY_H, SMALL_PX);
        fb.fill_rect(left, top as u32, KEY_W, KEY_H, BUTTON_BG);
        text::line(fb, left + 12, y, &[&b"y  "[..], PROVE_IT].concat(), BUTTON_FG, SMALL_PX);
        text::line(fb, left + KEY_W + 16, y, &[&b"n  "[..], NOT_NOW].concat(), MUTED, SMALL_PX);
        return top + KEY_H as i32 + 14;
    }
    let Some(run) = p.run.as_ref().filter(|r| r.listing == l.id) else { return top };
    let at = (left, room, top);
    let top = match (run.cancelled, working(run.phase), run.phase) {
        (true, ..) => lines(fb, STOPPED, MUTED, SMALL_PX, at),
        (_, Some(line), _) => lines(fb, line, ACCENT, SMALL_PX, at),
        (_, None, Phase::Failed(f, past)) => {
            let top = lines(fb, &said(f, &run.name, !run.release.is_empty()), DANGER, SMALL_PX, at);
            again(past).map_or(top, |a| lines(fb, a, MUTED, SMALL_PX, (left, room, top)))
        }
        _ => lines(fb, READY, OK, SMALL_PX, at),
    };
    top + 10
}

/// `words` wrapped to the pane, two lines at most; the y after them.
fn lines(fb: &mut PaintBuffer, words: &[u8], tone: u32, px: f32, at: (u32, u32, i32)) -> i32 {
    let (left, room, mut top) = at;
    for part in wrap(words, room, px).iter().take(2) {
        text::line(fb, left, top, part, tone, px);
        top += if px == BODY_PX { 22 } else { 20 };
    }
    top
}
