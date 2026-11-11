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

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::Rect;

use crate::ui::layout::{edge, pad, Layout};

const BTN: u32 = 38;
/// Half the play button's width.
const PLAY_HALF: u32 = 25;
const GAP: u32 = 12;

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
    Rect { x, y, w, h: scale().px(BTN) }
}

pub fn transport(l: &Layout, w: u32) -> Transport {
    let s = scale();
    let (btn, half, gap) = (s.px(BTN), s.px(PLAY_HALF), s.px(GAP));
    let y = l.bar.y + s.px(22);
    let cx = w / 2;
    let play_w = half * 2;
    let step = btn + gap;
    let inner = half + gap + btn / 2;
    let next = slot(cx + inner.saturating_sub(btn / 2), y, btn);
    let right = w.saturating_sub(edge() + pad());
    let note_x = next.x + next.w + step;
    Transport {
        prev: slot(cx.saturating_sub(inner + btn / 2), y, btn),
        play: slot(cx.saturating_sub(half), y, play_w),
        next,
        note: Rect { x: note_x, y, w: right.saturating_sub(note_x), h: btn },
    }
}
