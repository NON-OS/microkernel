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

use nonos_toolkit::widgets::Rect;

pub const TOP: u32 = 28;
pub const EDGE: u32 = 10;
pub const BAR_H: u32 = 64;

pub const PAD: u32 = 16;
/// The player's header under the title bar: back, the title and info.
pub const HEADER_H: u32 = 46;
const HEAD_BUTTON: u32 = 30;
const SCRUB_GRAB: u32 = 9;

/// Where the player screen puts its parts. The picture sits under the
/// header, so the header's buttons are never drawn over.
pub struct Layout {
    pub video: Rect,
    pub bar: Rect,
    pub scrub: Rect,
    pub elapsed_x: u32,
    pub remain_x: u32,
    pub back: Rect,
    pub info: Rect,
}

pub fn layout(w: u32, h: u32) -> Layout {
    let usable_h = h.saturating_sub(TOP).saturating_sub(EDGE);
    let bar_h = BAR_H.min(usable_h);
    let bar_y = h.saturating_sub(EDGE).saturating_sub(bar_h).max(TOP);
    let inner_w = w.saturating_sub(EDGE).saturating_sub(PAD).saturating_sub(PAD).max(1);
    let (video_y, side) = ((TOP + HEADER_H).min(bar_y), HEAD_BUTTON);
    let head_y = TOP + (HEADER_H - side) / 2;
    Layout {
        video: Rect { x: 0, y: video_y, w, h: bar_y - video_y },
        bar: Rect { x: 0, y: bar_y, w: w.saturating_sub(EDGE), h: bar_h },
        scrub: Rect { x: PAD, y: bar_y + 8, w: inner_w, h: 6 },
        elapsed_x: PAD,
        remain_x: w.saturating_sub(EDGE).saturating_sub(PAD),
        back: Rect { x: PAD.saturating_sub(6), y: head_y, w: side, h: side },
        info: Rect { x: w.saturating_sub(PAD + side), y: head_y, w: side, h: side },
    }
}

/// Where a click seeks: the groove `scrub`, and `SCRUB_GRAB` above and below.
pub fn seek_reach(scrub: Rect) -> Rect {
    let y = scrub.y.saturating_sub(SCRUB_GRAB);
    Rect { y, h: scrub.h + SCRUB_GRAB * 2, ..scrub }
}

/// The permille of the video a click at `x` on the groove seeks to, the
/// groove's first pixel the start and its last the end.
pub fn seek_permille(scrub: Rect, x: i32) -> u32 {
    let along = (x.max(0) as u32).saturating_sub(scrub.x);
    let span = scrub.w.saturating_sub(1).max(1) as u64;
    (along as u64 * 1000 / span).min(1000) as u32
}
