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

//! The band across the top: the install icon, the screen's title, and
//! which of the five steps this is.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::{draw, IconId};

use super::metrics::{HEADER_H, PAD, SMALL_PX, TITLE_PX};
use super::text::{right, top_of};
use super::{text, theme};
use crate::install::state::Screen;

const ICON: u32 = 28;

pub fn paint(fb: &mut PaintBuffer, screen: Screen, w: u32) {
    fb.fill_rect(0, 0, w, HEADER_H, theme::HEADER_BG);
    fb.fill_rect(0, HEADER_H - 1, w, 1, theme::RULE);
    draw(fb, IconId::Install, PAD, (HEADER_H - ICON) / 2, ICON, theme::ACCENT);
    let title_x = PAD + ICON + 16;
    text::line(fb, title_x, top_of(0, HEADER_H, TITLE_PX), screen.title(), theme::TITLE, TITLE_PX);
    let step = alloc::format!("step {} of 5", screen.step());
    right(fb, w - PAD, top_of(0, HEADER_H, SMALL_PX), &step, theme::MUTED, SMALL_PX);
}
