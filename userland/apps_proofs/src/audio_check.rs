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
//! The windows the audio player's proofs place controls in, the points of a
//! box they test, and the hover fed as the window feeds it.

use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, ButtonUp};
use nonos_toolkit::design::scale::scale;

use crate::audio_ui::geometry::Rect;
use crate::audio_ui::hover::Hover;
use crate::audio_ui::shown::Shown;
use crate::audio_ui::view::View;

/// `w` by `h` logical pixels at the display scale in force.
pub fn dims(w: u32, h: u32) -> (u32, u32) {
    (scale().px(w), scale().px(h))
}

/// A 1440 by 900 window on `view` at the display scale in force, with a
/// library, a queue and downloads.
pub fn window(view: View) -> Shown {
    let acts = vec![vec!["Cancel"], vec!["Play", "Remove"], vec![], vec!["Resume", "Remove"]];
    let (dims, rows, tracks, queue) = (dims(1440, 900), 40, 40, 6);
    let (scroll, query, address, finished) = (0, true, false, true);
    Shown { view, dims, scroll, query, address, rows, tracks, queue, acts, finished }
}

/// Every view; the Library also scrolled, Home also empty, Search also with
/// an address and with nothing typed; and a window too narrow for the rail.
pub fn windows() -> Vec<Shown> {
    use View::*;
    let mut all: Vec<Shown> =
        [Home, Library, Search, NowPlaying, Downloads, Settings].map(window).into();
    all.push(Shown { scroll: 7, ..window(Library) });
    all.push(Shown { tracks: 0, rows: 0, ..window(Home) });
    all.push(Shown { address: true, rows: 0, ..window(Search) });
    all.push(Shown { query: false, ..window(Search) });
    all.push(Shown { dims: dims(1000, 700), ..window(Downloads) });
    all
}

/// The middle and the four corners of `r`.
pub fn points(r: Rect) -> [(i32, i32); 5] {
    let (x1, y1) = (r.right() - 1, r.bottom() - 1);
    [(r.cx(), r.cy()), (r.x, r.y), (x1, r.y), (r.x, y1), (x1, y1)]
}

/// Whether two boxes share a pixel.
pub fn meets(a: Rect, b: Rect) -> bool {
    a.x < b.right() && b.x < a.right() && a.y < b.bottom() && b.y < a.bottom()
}

/// Feeds the event and names the control under the pointer again, as the
/// window does; true when the window would repaint.
pub fn feed(h: &mut Hover, s: &Shown, kind: InputKind, (x, y): (i32, i32)) -> bool {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    let ev = InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns };
    let fed = h.feed(&ev);
    let held = kind == ButtonUp && h.over.is_some();
    fed && (h.settle(s) || held)
}
