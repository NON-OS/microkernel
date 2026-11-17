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

//! The columns a table can show, their titles and fixed widths, and the
//! shared table's column for each.

use alloc::vec::Vec;

use nonos_toolkit::widgets::Col as Column;

use super::metrics::{COL_AUTH_W, COL_CPU_W, COL_MEM_W, COL_PID_W, COL_RATE_W, COL_STATE_W};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Col {
    Name,
    Pid,
    State,
    Cpu,
    Mem,
    Ipc,
    Sysc,
    Auth,
}

pub const COLS_OVERVIEW: [Col; 7] =
    [Col::Name, Col::Pid, Col::Cpu, Col::Mem, Col::Ipc, Col::Sysc, Col::Auth];
// Faults and uptime are not columns: they stay in the inspector, so with it
// docked the pane holds eight columns and a name that can still be read.
pub const COLS_FULL: [Col; 8] =
    [Col::Name, Col::Pid, Col::State, Col::Cpu, Col::Mem, Col::Ipc, Col::Sysc, Col::Auth];

// Name is the one flex column, so it declares no budget of its own and takes
// whatever the fixed columns leave.
pub fn fixed_w(col: Col) -> u32 {
    match col {
        Col::Name => 0,
        Col::Pid => COL_PID_W,
        Col::State => COL_STATE_W,
        Col::Cpu => COL_CPU_W,
        Col::Mem => COL_MEM_W,
        Col::Ipc | Col::Sysc => COL_RATE_W,
        Col::Auth => COL_AUTH_W,
    }
}

/// The title the header shows over `col`.
pub fn title(col: Col) -> &'static str {
    match col {
        Col::Name => "NAME",
        Col::Pid => "PID",
        Col::State => "STATE",
        Col::Cpu => "CPU",
        Col::Mem => "MEMORY",
        Col::Ipc => "IPC/S",
        Col::Sysc => "SYSC/S",
        Col::Auth => "AUTHORITY",
    }
}

/// The shared table's column for `col`: Name takes the share the fixed
/// columns leave, and a column of numbers keeps to the right so its digits
/// line up.
pub fn spec(col: Col) -> Column<'static> {
    let c = match fixed_w(col) {
        0 => Column::new(title(col)),
        w => Column::new(title(col)).fixed(w),
    };
    match col {
        Col::Name | Col::State | Col::Auth => c,
        _ => c.right(),
    }
}

/// The shared table's columns for `cols`, in the same order.
pub fn specs(cols: &[Col]) -> Vec<Column<'static>> {
    cols.iter().map(|c| spec(*c)).collect()
}
