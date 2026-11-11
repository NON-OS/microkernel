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
use nonos_toolkit::widgets::{Col as Column, Table};

use crate::pm::format::{mem_human, percent, state_label, u32_decimal};
use crate::pm::format_sys::rate_human;
use crate::pm::state::{Row, State};

use super::chrome::Rect;
use super::metrics::RISK_SLOT_H;
use super::table_geom::Col;
use super::{badge, risk_strip, text};

/// The text of each of `cols`, whose shared columns are `specs`, in a table
/// drawn in `at`, for each row `shown`.
pub fn cells(
    state: &State,
    (cols, specs): (&[Col], &[Column]),
    at: Rect,
    shown: &[&Row],
) -> Vec<Vec<String>> {
    let head = Table::new(specs);
    let room = |c| head.head_rect(at, c).map_or(0, |h| Table::text_rect(h).w);
    let rooms: Vec<u32> = (0..cols.len()).map(room).collect();
    shown.iter().map(|row| line(state, (cols, &rooms), row)).collect()
}

/// One row's cells. Authority is left blank, since its letters are drawn
/// over the cell.
fn line(state: &State, (cols, rooms): (&[Col], &[u32]), row: &Row) -> Vec<String> {
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
    cols.iter().zip(rooms).map(|(col, room)| cell(*col, *room)).collect()
}

/// Over the rows `table` drew in `at` from whole-list index `first`, `shown`
/// with the text `cells` it drew: the shield after a protected name and the
/// four risk letters under Authority, each in its own colour.
pub fn marks(
    state: &State,
    fb: &mut PaintBuffer,
    (table, at): (&Table, Rect),
    (cols, first): (&[Col], usize),
    shown: &[&Row],
    cells: &[Vec<String>],
) {
    for (k, (row, line)) in shown.iter().zip(cells).enumerate() {
        for (c, (col, drawn)) in cols.iter().zip(line).enumerate() {
            let Some(cell) = table.cell_rect(at, first + k, c) else { continue };
            let x = Table::text_rect(cell).x;
            match col {
                Col::Name if state.is_protected(row) => badge::shield(fb, x, cell.y, drawn),
                Col::Auth => {
                    let top = cell.y + cell.h.saturating_sub(RISK_SLOT_H) / 2;
                    risk_strip::paint(fb, x, top, row.caps);
                }
                _ => {}
            }
        }
    }
}
