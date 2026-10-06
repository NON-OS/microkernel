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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::omnibox::center_x;
use crate::browser::paint::home_page::{constants, shortcut_data};

pub fn shortcut(fb: &mut PaintBuffer, i: u32) {
    let s = &shortcut_data::SHORTCUTS[i as usize];
    let cx = center_x(fb.width, shortcut_data::SHORTCUTS.len() as u32, i);
    let bx = cx.saturating_sub(constants::BADGE / 2);
    let (b, y) = (constants::BADGE, constants::BADGE_Y);
    fb.fill_rect(bx, y, b, b, s.color);
    for (px, py) in [(bx, y), (bx + b - 6, y), (bx, y + b - 6), (bx + b - 6, y + b - 6)] {
        fb.fill_rect(px, py, 6, 6, constants::PAGE_BG);
    }
    let badge = core::str::from_utf8(s.badge).unwrap_or("");
    let bw = fb.measure_ttf(badge, 26.0);
    fb.text_ttf(cx as i32 - bw / 2, (y + 14) as i32, badge, constants::WHITE, 26.0);
    let label = core::str::from_utf8(s.label).unwrap_or("");
    let lw = fb.measure_ttf(label, 15.0);
    fb.text_ttf(cx as i32 - lw / 2, 366, label, constants::FG, 15.0);
}
