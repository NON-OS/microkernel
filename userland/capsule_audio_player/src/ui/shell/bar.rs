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

//! Transport-bar geometry. The painter and the hit-test both read `bar`, so a
//! click lands on the control that was actually drawn under the pointer.

use super::super::geometry::Rect;
use super::super::metrics::{metrics, px};

const THUMB: i32 = 46;
const PLAY_R: i32 = 21;
const VOL_W: i32 = 108;
const STEP: i32 = 44;

pub struct Bar {
    pub thumb: Rect,
    pub shuffle: Rect,
    pub prev: Rect,
    pub play: Rect,
    pub next: Rect,
    pub repeat: Rect,
    pub scrub: Rect,
    pub volume: Rect,
    pub speaker: Rect,
}

/// The play disc's radius at the display scale.
pub fn play_r() -> i32 {
    px(PLAY_R)
}

pub fn bar(r: Rect) -> Bar {
    let (m, cy, step, b, t) = (metrics(), r.cy(), px(STEP), px(16), px(THUMB));
    let (hub, pr, vol_w) = (r.cx() - px(40), play_r(), px(VOL_W));
    let btn = |i: i32| Rect::new(hub + i * step - b, cy - b, b * 2, b * 2);
    let scrub_x = hub + 2 * step + m.s6 + b;
    let vol_x = r.right() - m.s6 - vol_w;
    let (gy, gh, spk) = (cy - px(3), px(6), px(26));
    Bar {
        thumb: Rect::new(r.x + m.s6, cy - t / 2, t, t),
        shuffle: btn(-2),
        prev: btn(-1),
        play: Rect::new(hub - pr, cy - pr, pr * 2, pr * 2),
        next: btn(1),
        repeat: btn(2),
        scrub: Rect::new(scrub_x, gy, (vol_x - m.s5 - spk - scrub_x).max(0), gh),
        volume: Rect::new(vol_x, gy, vol_w, gh),
        speaker: Rect::new(vol_x - spk, cy - px(11), px(22), px(22)),
    }
}
