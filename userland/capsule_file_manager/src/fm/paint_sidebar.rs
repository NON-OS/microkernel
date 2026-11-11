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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Rect, SectionHeader};

use super::event_side::shown_rows;
use super::layout::{hair, metrics};
use super::sidebar_brand::paint_brand;
use super::sidebar_metrics::rail;
use super::sidebar_model::SideKind;
use super::sidebar_row_paint::{nav_row, rule};
use super::sidebar_storage::paint_storage;
use super::state::State;

/// Draws the rail from the one layout pass `side_at` also reads, so every mark
/// on screen has a slot behind it and no slot is left undrawn. A row is drawn
/// with its index in that pass, which is how the hover names it.
pub fn paint_sidebar(state: &State, fb: &mut PaintBuffer) {
    let (h, r, (side, line), rl) = (fb.height, roles(), (metrics().sidebar_w, hair()), rail());
    draw::fill(fb, Rect { x: 0, y: 0, w: side, h }, r.sunken);
    draw::fill(fb, Rect { x: side - line, y: 0, w: line, h }, r.border);
    paint_brand(fb);
    for (i, row) in shown_rows(state).iter().enumerate() {
        match row.kind {
            SideKind::Label => {
                let at = Rect { x: rl.x + rl.pad, y: row.y, w: rl.w - rl.pad * 2, h: row.h };
                SectionHeader::new(&row.label).draw(fb, at);
            }
            SideKind::Rule => rule(fb, row.y, row.h),
            SideKind::Nav => nav_row(state, fb, i, row),
            SideKind::Storage => paint_storage(state, fb, i, row.y),
        }
    }
}
