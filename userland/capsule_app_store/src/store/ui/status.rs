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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Ink, Label, Pill, Rect, Separator, Tone};

use crate::store::progress::Progress;
use crate::store::state::State;
use crate::store::tier_fit::hidden_line;

use super::bytes::as_str;
use super::metrics::metrics;

pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let m = metrics();
    let (h, pad) = (m.status_h, m.status_pad_x);
    let strip = Rect { x: 0, y: state.fb_h.saturating_sub(h), w: state.fb_w, h };
    draw::fill(fb, strip, roles().sunken);
    let rule = Separator::new();
    rule.draw(fb, Rect { h: rule.size().1, ..strip });
    let lane = Rect { x: pad, w: state.fb_w.saturating_sub(pad * 2), ..strip };
    /*
     * Enter's word is the card's button, so the hint never disagrees with it.
     * With no NONOS disk Enter does nothing for a model, so it is not offered.
     */
    let current = state.current();
    let enter = current.map_or(&b"Install"[..], |l| l.progress.button(l.ready));
    let idle = current.is_some_and(|l| l.progress.needs_installed_system());
    let mut keys = alloc::vec::Vec::with_capacity(96);
    keys.extend_from_slice(b"up/down select    ");
    if !idle {
        keys.extend_from_slice(b"Enter ");
        keys.extend(enter.iter().map(u8::to_ascii_lowercase));
        keys.extend_from_slice(b"    ");
    }
    /* A large tier download through Nym or Anyone may be taken direct, by choice. */
    if current.is_some_and(|l| crate::store::install::direct_offered_for(state, l)) {
        keys.extend_from_slice(b"d download direct    ");
    }
    if current.is_some_and(|l| l.progress == Progress::Installed) {
        keys.extend_from_slice(b"u remove    ");
    }
    keys.extend_from_slice(b"/ search    r refresh    Esc close");
    /* The answer to the last request sits opposite the keys. */
    let taken = answer(fb, state, lane);
    let room = lane.w.saturating_sub(if taken == 0 { 0 } else { taken + pad });
    Label::new(as_str(&keys)).ink(Ink::Muted).draw(fb, Rect { w: room, ..lane });
}

/// The answer to the last request as an accent pill at the right end of
/// `lane`, or else the count of hidden tiers, and the width it took.
fn answer(fb: &mut PaintBuffer, state: &State, lane: Rect) -> u32 {
    let right = |w: u32| Rect { x: lane.x + lane.w.saturating_sub(w), w: w.min(lane.w), ..lane };
    if let Some(asked) = state.asked {
        let pill = Pill::new(as_str(asked.label())).tone(Tone::Accent);
        let at = right(pill.size().0);
        pill.draw(fb, at);
        return at.w;
    }
    /* The tiers that do not fit are counted, not shown (`tier_fit`). */
    let Some(line) = hidden_line(state.hidden()) else { return 0 };
    let label = Label::new(&line).ink(Ink::Muted);
    let at = right(label.size().0);
    label.draw(fb, at);
    at.w
}
