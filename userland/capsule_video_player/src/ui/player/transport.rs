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

use crate::ui::layout::{Layout, Rect, EDGE, PAD};

pub const BTN: u32 = 38;
pub const HEX_R: u32 = 25;
pub const GAP: u32 = 12;

/// The bar's buttons: back ten seconds, play or pause, forward ten seconds.
/// The player decodes pictures only (Motion-JPEG, no sound track), and has no
/// subtitles, playlist or full-screen mode, so it draws no volume, captions,
/// shuffle, repeat, picture-in-picture or full-screen buttons. `note` is where
/// the bar says so.
pub struct Transport {
    pub prev: Rect,
    pub play: Rect,
    pub next: Rect,
    pub note: Rect,
}

fn slot(x: u32, y: u32, w: u32) -> Rect {
    Rect { x, y, w, h: BTN }
}

pub fn transport(l: &Layout, w: u32) -> Transport {
    let y = l.bar.y + 22;
    let cx = w / 2;
    let hex = HEX_R * 2;
    let step = BTN + GAP;
    let inner = HEX_R + GAP + BTN / 2;
    let next = slot(cx + inner.saturating_sub(BTN / 2), y, BTN);
    let right = w.saturating_sub(EDGE + PAD);
    let note_x = next.x + next.w + step;
    Transport {
        prev: slot(cx.saturating_sub(inner + BTN / 2), y, BTN),
        play: slot(cx.saturating_sub(HEX_R), y, hex),
        next,
        note: Rect { x: note_x, y, w: right.saturating_sub(note_x), h: BTN },
    }
}
