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
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::icons::draw;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text;

use crate::settings::section::Section;
use crate::settings::section_text::{subtitle, title};

use super::icon_table::icon;
use super::metrics::metrics;

pub fn paint(fb: &mut PaintBuffer, section: Section, top: i32) {
    let (r, m) = (roles(), metrics());
    let x = m.pane_pad_x;
    draw(fb, icon(section), x, top.max(0) as u32, m.head_icon, r.accent.argb());
    let text_x = x + m.head_icon + m.head_icon_gap;
    text::left(fb, text_x, top, title(section), r.text, m.title_px);
    let below = top + line_height(m.title_px) + m.head_line_gap as i32;
    text::left(fb, text_x, below, subtitle(section), r.muted, m.subtitle_px);
}
