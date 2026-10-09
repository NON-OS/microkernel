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

//! The left panel's lines: a numbered step, and a centred caption.

use nonos_app_skeleton::PaintBuffer;

use nonos_brand::palette::{CYAN, TEXT_3};
use nonos_brand::{label, label_w, line_h, Face};

use super::layout::FullLayout;

#[allow(clippy::too_many_arguments)]
pub(super) fn row(
    fb: &mut PaintBuffer,
    l: &FullLayout,
    x: u32,
    y: u32,
    num: &str,
    name: &str,
    c: u32,
    here: bool,
) {
    let px = l.label_px;
    let dy = l.step_h.saturating_sub(line_h(Face::Mono, px)) / 2;
    if here {
        let bar = l.scale.px(2);
        let bar_h = l.step_h.saturating_sub(2 * dy).max(bar);
        fb.fill_round(x.saturating_sub(2 * l.unit) + bar, y + dy, bar, bar_h, 1, CYAN);
    }
    let n = label(fb, x, y + dy, num, if here { CYAN } else { TEXT_3 }, px);
    label(fb, n + l.unit + l.unit / 4, y + dy, name, c, px);
}

pub(super) fn centred(fb: &mut PaintBuffer, l: &FullLayout, y: u32, s: &str, c: u32) {
    label(fb, l.panel_w.saturating_sub(label_w(s, l.label_px)) / 2, y, s, c, l.label_px);
}
