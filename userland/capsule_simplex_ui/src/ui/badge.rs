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

//! The unread count beside a chat in the list.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::{measure_ttf, PaintBuffer};

use super::metrics::{Metrics, SMALL_PX};
use super::theme::Colors;

/// Draws the count `n` right aligned at `right`; its width.
pub fn badge(fb: &mut PaintBuffer, right: u32, y: u32, n: u32, m: Metrics, c: &Colors) -> u32 {
    let text = if n > 99 { String::from("99+") } else { format!("{n}") };
    let h = m.px(20);
    let w = (measure_ttf(&text, SMALL_PX).max(0) as u32 + m.px(12)).max(h);
    let x = right.saturating_sub(w);
    fb.fill_round(x, y, w, h, h / 2, c.accent);
    let tx = x + (w - measure_ttf(&text, SMALL_PX).max(0) as u32) / 2;
    fb.text_ttf(tx as i32, (y + m.px(3)) as i32, &text, c.on_accent, SMALL_PX);
    w
}
