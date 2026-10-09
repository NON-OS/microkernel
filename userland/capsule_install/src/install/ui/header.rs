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

//! The band across the top: the NØNOS mark, the screen's title, and which
//! of the five steps this is, in the brand's mono.

use alloc::format;

use nonos_app_skeleton::PaintBuffer;

use super::metrics::Metrics;
use super::text::top_of;
use super::{text, theme};
use crate::install::state::Screen;
use nonos_brand::{label, label_w, mark};

pub fn paint(fb: &mut PaintBuffer, m: &Metrics, screen: Screen, w: u32) {
    let band = m.header_h;
    fb.fill_rect(0, 0, w, band, theme::HEADER_BG);
    fb.fill_rect(0, band - 1, w, 1, theme::RULE);
    let mark_h = m.scale.px(56);
    mark(fb, m.pad + m.inset, band / 2, mark_h, theme::ACCENT, 140);
    let title_x = m.pad + 5 * m.unit + m.unit / 2;
    let top = top_of(0, band, m.title_px);
    text::title(fb, title_x, top, screen.title(), theme::TITLE, m.title_px);
    let step = format!("{:02} / 05", screen.step());
    let sx = (w - m.pad).saturating_sub(label_w(&step, m.small_px));
    label(fb, sx, top_of(0, band, m.small_px), &step, theme::MUTED, m.small_px);
}
