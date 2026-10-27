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

//! The row an installer on an installed NØNOS offers on its first screen:
//! prepare this computer for an update from a new stick, so the new
//! release can keep the data (`install/handover`): what it is for, and
//! under it what the kernel answered when P was pressed.

use nonos_app_skeleton::PaintBuffer;

use crate::install::handover::answer;
use crate::install::state::State;
use crate::install::ui::metrics::Metrics;
use crate::install::ui::text::{right, top_of};
use crate::install::ui::wrap::{paragraph, Ink};
use crate::install::ui::{text, theme};

const ROW: &str = "Prepare this computer for an update";
const WHY: &str = "So an update from a new NØNOS stick can keep your data.";

/// Paints the row from `y` across `w`.
pub fn prepare_row(state: &State, fb: &mut PaintBuffer, m: &Metrics, x: u32, y: u32, w: u32) {
    fb.fill_round(x, y, w, m.row_h, m.radius, theme::SELECTED_BG);
    let top = top_of(y + m.unit / 2, m.line_h, m.body_px);
    text::line(fb, x + m.inset, top, ROW, theme::TITLE, m.body_px);
    right(fb, x + w - m.inset, top, "press P", theme::MUTED, m.body_px);
    let under = top_of(y + m.unit / 2 + m.line_h, m.line_h, m.body_px);
    text::line(fb, x + m.inset, under, WHY, theme::MUTED, m.body_px);
    if let Some((said, done)) = state.handover.map(answer) {
        let ink = Ink::body(m, if done { theme::OK } else { theme::WARN });
        paragraph(fb, x, y + m.row_h + m.unit, w, &said, ink);
    }
}
