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

//! The left-to-right cell walk. Pill widths follow their measured labels, so a
//! longer style name pushes the toggles along and the hit-test follows, because
//! it reads back the very cells this produced.

use alloc::vec::Vec;
use nonos_app_skeleton::measure_ttf;

use super::items::{RibbonItem, ICON_COUNT, TOGGLES};
use super::metrics::{lengths, RibbonCell};
use crate::editor::layout::{activity_w, chrome_px};

pub(super) fn cells(labels: &[&str; 3]) -> Vec<RibbonCell> {
    let (mut out, l, px) = (Vec::new(), lengths(), chrome_px());
    let mut x = activity_w() + l.bar_pad;
    for (p, label) in labels.iter().enumerate() {
        let w = measure_ttf(label, px).max(0) as u32 + l.pill_pad * 2 + l.chevron_w;
        out.push(RibbonCell { x0: x, x1: x + w, item: RibbonItem::Pill(p) });
        x += w + l.gap;
    }
    x += l.sep_gap;
    for t in 0..TOGGLES.len() {
        out.push(RibbonCell { x0: x, x1: x + l.square_w, item: RibbonItem::Toggle(t) });
        x += l.square_w + l.gap;
    }
    x += l.sep_gap;
    for k in 0..ICON_COUNT {
        out.push(RibbonCell { x0: x, x1: x + l.square_w, item: RibbonItem::Icon(k) });
        x += l.square_w + l.gap;
    }
    out
}
