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

//! What the install erases and writes, one row each, read off the plan the
//! write follows; or, for a disk that cannot take NONOS, why not.

use nonos_app_skeleton::PaintBuffer;
use nonos_disk::describe;

use crate::install::state::State;
use crate::install::ui::metrics::Metrics;
use crate::install::ui::theme;
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::wrap::{paragraph, Ink};

/// Lines the card for a disk that cannot take NONOS holds.
const REFUSED_LINES: u32 = 3;

/// Paints the card at `y` and returns how many lines it holds, so the
/// screen places what follows it (confirm_stack); none when nothing is
/// prepared yet.
pub fn written(state: &State, fb: &mut PaintBuffer, m: &Metrics, x: u32, y: u32, w: u32) -> u32 {
    match &state.prepared {
        Some(Ok(p)) => {
            let rows = describe(&p.plan, &p.carried);
            let lines = rows.len() as u32;
            let mut r = card(fb, m, x, y, w, m.card_h(lines), "what is erased and written");
            for row in &rows {
                r = kv(fb, m, x + m.inset, r, w - 2 * m.inset, row.label, &row.value, false);
            }
            lines
        }
        Some(Err(why)) => {
            let h = m.card_h(REFUSED_LINES);
            let inner = card(fb, m, x, y, w, h, "this disk cannot take NONOS");
            paragraph(fb, x + m.inset, inner, w - 2 * m.inset, why, Ink::body(m, theme::DANGER));
            REFUSED_LINES
        }
        None => 0,
    }
}
