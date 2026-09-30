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
use crate::install::ui::metrics::LINE_H;
use crate::install::ui::theme;
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::wrap::{paragraph, Ink};

/// Paints the card at `y` and returns the y below it.
pub fn written(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) -> u32 {
    match &state.prepared {
        Some(Ok(p)) => {
            let rows = describe(&p.plan, &p.carried);
            let h = rows.len() as u32 * LINE_H + 40;
            let mut r = card(fb, x, y, w, h, "what is erased and written");
            for row in &rows {
                r = kv(fb, x + 16, r, w - 32, row.label, &row.value, false);
            }
            y + h
        }
        Some(Err(why)) => {
            let inner = card(fb, x, y, w, 3 * LINE_H + 40, "this disk cannot take NONOS");
            paragraph(fb, x + 16, inner, w - 32, why, Ink::body(theme::DANGER));
            y + 3 * LINE_H + 40
        }
        None => y,
    }
}
