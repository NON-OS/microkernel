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

const TOP: u32 = 28;
const EDGE: u32 = 10;
const BAR_H: u32 = 64;

const PAD: u32 = 16;
/// The player's header under the title bar: back, the title and info.
const HEADER_H: u32 = 46;
const HEAD_BUTTON: u32 = 30;
const SCRUB_GRAB: u32 = 9;

/// The band over every page, in drawing pixels at the scale in force.
pub fn top() -> u32 {
    scale().px(TOP)
}

/// The player's right and bottom margin, in drawing pixels.
pub fn edge() -> u32 {
    scale().px(EDGE)
}

/// The player's side padding, in drawing pixels.
pub fn pad() -> u32 {
    scale().px(PAD)
}

/// The player's header height, in drawing pixels.
pub fn header_h() -> u32 {
    scale().px(HEADER_H)
}

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
    let (s, top, edge, pad, head) = (scale(), top(), edge(), pad(), header_h());
    let usable_h = h.saturating_sub(top).saturating_sub(edge);
    let bar_h = s.px(BAR_H).min(usable_h);
    let bar_y = h.saturating_sub(edge).saturating_sub(bar_h).max(top);
    let inner_w = w.saturating_sub(edge).saturating_sub(pad).saturating_sub(pad).max(1);
    let (video_y, side) = ((top + head).min(bar_y), s.px(HEAD_BUTTON));
    let head_y = top + head.saturating_sub(side) / 2;
    Layout {
        video: Rect { x: 0, y: video_y, w, h: bar_y - video_y },
        bar: Rect { x: 0, y: bar_y, w: w.saturating_sub(edge), h: bar_h },
        scrub: Rect { x: pad, y: bar_y + s.px(8), w: inner_w, h: s.px(6) },
        elapsed_x: pad,
        remain_x: w.saturating_sub(edge).saturating_sub(pad),
        back: Rect { x: pad.saturating_sub(s.px(6)), y: head_y, w: side, h: side },
        info: Rect { x: w.saturating_sub(pad + side), y: head_y, w: side, h: side },
    }
}

/// Where a click seeks: the groove `scrub`, and `SCRUB_GRAB` above and below.
pub fn seek_reach(scrub: Rect) -> Rect {
    let grab = scale().px(SCRUB_GRAB);
    let y = scrub.y.saturating_sub(grab);
    Rect { y, h: scrub.h + grab * 2, ..scrub }
}

/// The permille of the video a click at `x` on the groove seeks to, the
/// groove's first pixel the start and its last the end.
pub fn seek_permille(scrub: Rect, x: i32) -> u32 {
    let along = (x.max(0) as u32).saturating_sub(scrub.x);
    let span = scrub.w.saturating_sub(1).max(1) as u64;
    (along as u64 * 1000 / span).min(1000) as u32
}
