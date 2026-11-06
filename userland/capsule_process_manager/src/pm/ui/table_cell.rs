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

//! What each cell of a process table says, and the marks drawn over it that
//! the shared table, which draws text alone, cannot.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::pm::format::{mem_human, percent, state_label, u32_decimal};
use crate::pm::format_sys::rate_human;
use crate::pm::state::{Row, State};

use super::chrome::Rect;
use super::metrics::RISK_SLOT_H;
use super::table_geom::{ink, row_h, row_y, Col};
use super::{badge, risk_strip, text};

/// The text of each of `cols`, laid out as `spans`, for each row `shown`.
pub fn cells(state: &State, at: (&[Col], &[(u32, u32)]), shown: &[&Row]) -> Vec<Vec<String>> {
    shown.iter().map(|row| line(state, at, row)).collect()
}

/// One row's cells. Authority is left blank, since its letters are drawn
/// over the cell.
fn line(state: &State, (cols, spans): (&[Col], &[(u32, u32)]), row: &Row) -> Vec<String> {
    let mut buf = [0u8; 24];
    let mut cell = |col: Col, room: u32| {
        let n = match col {
            Col::Name => return badge::name(row.name(), room, state.is_protected(row)),
            Col::State => return String::from(text::valid(state_label(row.state))),
            Col::Auth => 0,
            Col::Pid => u32_decimal(row.pid, &mut buf),
            Col::Cpu => percent(row.cpu_pct, &mut buf),
            Col::Mem => mem_human(row.mem_kb, &mut buf),
            Col::Ipc => rate_human(row.ipc_ps, &mut buf),
            Col::Sysc => rate_human(row.sysc_ps, &mut buf),
        };
        String::from(text::valid(&buf[..n]))
    };
    cols.iter().zip(spans).map(|(col, span)| cell(*col, ink(*span).1)).collect()
}

/// Over the rows drawn from the table's corner `r`, `shown` with the text
/// `cells` the table drew: the shield after a protected name and the four
/// risk letters under Authority, each in its own colour.
pub fn marks(
    state: &State,
    fb: &mut PaintBuffer,
    r: &Rect,
    (cols, spans): (&[Col], &[(u32, u32)]),
    shown: &[&Row],
    cells: &[Vec<String>],
) {
    for (k, (row, line)) in shown.iter().zip(cells).enumerate() {
        let y = r.y + row_y(k);
        for ((col, span), cell) in cols.iter().zip(spans).zip(line) {
            let x = r.x + ink(*span).0;
            match col {
                Col::Name if state.is_protected(row) => badge::shield(fb, x, y, cell),
                Col::Auth => {
                    let top = y + row_h().saturating_sub(RISK_SLOT_H) / 2;
                    risk_strip::paint(fb, x, top, row.caps);
                }
                _ => {}
            }
        }
    }
}
