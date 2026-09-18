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

//! What each process has cost the kernel, counted where it happens.
//!
//! Syscalls, messages sent and received, page faults and context switches
//! are counted per pid in fixed slots, the same shape as the tick table, so
//! a hot path pays one relaxed atomic add and never a lock. System totals
//! live beside them, with the interrupt count and the idle ticks that were
//! previously charged to whatever process last held the processor. All of
//! it is read by `MkProcStat`, which is how a monitor shows real rates.

mod counters;
mod idle;
mod kind;
mod split;
mod totals;

pub use counters::{bump, clear, snapshot, Snapshot};
pub use idle::{idle_enter, idle_leave, idle_ticks, is_idle, tick_idle};
pub use kind::Kind;
pub use split::{set_tick_origin, tick_charge};
pub use totals::{bump_total, totals, Total, Totals};
