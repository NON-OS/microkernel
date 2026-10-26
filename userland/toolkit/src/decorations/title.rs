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
//! The frame's measures at one to one (HANDOFF/ui/identity.md, Window
//! The window title: left after its inset, cut with an ellipsis before the
//! accessory or the controls, dimmed when the window is not focused.

use super::accessory::accessory_rect_at;
use super::control_rect::controls_left_at;
use super::frame_rect::{frame_rect_at, titlebar_h_at};
use super::metrics::{ACCESSORY_INSET, TITLE_INSET, TITLE_PX};
use super::palette::title_ink;
use super::scale::{at, px_at};
use crate::paint::{measure_ttf, PaintBuffer};
use crate::text::fit_end;

pub(super) fn draw_title(
    fb: &mut PaintBuffer,
    maximized: bool,
    title: &[u8],
    focused: bool,
    accessory_w: u32,
    q: u32,
) {
    let Ok(text) = core::str::from_utf8(title) else {
        return;
    };
    let (w, h) = (fb.width, fb.height);
    let f = frame_rect_at(w, h, maximized, q);
    let px = px_at(TITLE_PX, q);
    let x = f.x + at(TITLE_INSET, q);
    let end = match accessory_rect_at(w, h, maximized, accessory_w, q) {
        Some(a) => a.x,
        None => controls_left_at(w, h, maximized, q),
    };
    let room = end.saturating_sub(at(ACCESSORY_INSET, q)).saturating_sub(x);
    let shown = fit_end(text, room, |s| measure_ttf(s, px).max(0) as u32);
    let y = f.y + (titlebar_h_at(q) / 2).saturating_sub(px as u32 / 2);
    fb.text_ttf(x as i32, y as i32, &shown, title_ink(focused), px);
}
