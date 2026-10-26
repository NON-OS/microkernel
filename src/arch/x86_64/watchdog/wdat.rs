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

//! The watchdog firmware describes in WDAT, beside the TCO one: what it is
//! and what it reads now, for the kernel. Queries run whenever asked; no
//! action that writes (RESET, SET_COUNTDOWN) is offered, so nothing here
//! can start or stop one.

use crate::arch::x86_64::acpi::parser::other::wdat_action as act;
use crate::arch::x86_64::acpi::parser::other::{wdat_action, wdat_table};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WdatStatus {
    pub period_ms: u32,
    pub min_count: u32,
    pub max_count: u32,
    pub firmware_enabled: bool,
    pub running: Option<bool>,
    /// The countdown left, in milliseconds.
    pub left_ms: Option<u64>,
    pub instructions: usize,
    pub malformed: u32,
}

/// None when firmware has no WDAT.
pub fn wdat_status() -> Option<WdatStatus> {
    let w = wdat_table()?;
    let read = |a| wdat_action(a, 0, false).ok();
    let period = u64::from(w.period_ms);
    Some(WdatStatus {
        period_ms: w.period_ms,
        min_count: w.min_count,
        max_count: w.max_count,
        firmware_enabled: w.enabled,
        running: read(act::QUERY_RUNNING).map(|v| v != 0),
        left_ms: read(act::QUERY_CURRENT_COUNTDOWN).map(|c| c.saturating_mul(period)),
        instructions: w.entries.len(),
        malformed: w.malformed,
    })
}
