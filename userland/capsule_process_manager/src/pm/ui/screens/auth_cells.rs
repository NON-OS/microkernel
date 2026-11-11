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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::{truncate, width};
use nonos_toolkit::widgets::{Rect, Table};

use super::super::matrix_geom::{self as geom, MATRIX};
use super::super::table_geom::row_h;
use super::super::text::valid;
use super::super::tint::cap_tint;

const DOT_RADIUS: u32 = 3;
const RING_PAD: u32 = 2;

/// Where each authority's mark is centred across a grid `head` draws in `at`:
/// under its title, as the shared table draws it cut to its column.
pub fn centres(head: &Table, at: Rect) -> Vec<u32> {
    let px = scale().font(TextRole::Body.px());
    let under = |label: &[u8], ink: Rect| ink.x + width(&truncate(valid(label), ink.w, px), px) / 2;
    let title = |(k, (_, label)): (usize, &(u64, &[u8]))| {
        head.head_rect(at, k + 1).map(|h| under(label, Table::text_rect(h)))
    };
    MATRIX.iter().enumerate().filter_map(title).collect()
}

// A held authority is a filled dot in its risk-class colour; an unheld one leaves
// a hairline tick, so an empty column still reads as a column rather than as a
// gap. The ring is the finding this screen exists to surface: a sensitive grant
// on a process the kernel does not protect. The class tone, danger and border
// are all opaque, and fill_round strokes blend anyway, so nothing punches the
// row.
pub fn paint(fb: &mut PaintBuffer, y: u32, centres: &[u32], caps: u64, protected: bool) {
    let (c, s) = (roles(), scale());
    let (dot, radius, pad, hair) =
        (s.px(geom::CELL_DOT), s.px(DOT_RADIUS), s.px(RING_PAD), s.hair());
    for ((bit, _), centre) in MATRIX.iter().zip(centres) {
        let cx = centre.saturating_sub(dot / 2);
        let cy = y + row_h().saturating_sub(dot) / 2;
        if caps & bit == 0 {
            fb.fill_rect(cx, cy + dot / 2, dot, hair, c.border.argb());
            continue;
        }
        fb.fill_round(cx, cy, dot, dot, radius, cap_tint(*bit).colour(&c).argb());
        if !protected {
            let ring = dot + pad * 2;
            let (rx, ry) = (cx.saturating_sub(pad), cy.saturating_sub(pad));
            fb.stroke_round(rx, ry, ring, ring, radius + pad, hair, c.danger.argb());
        }
    }
}
