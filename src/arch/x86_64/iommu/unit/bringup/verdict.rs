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

//! The lines bring-up is entitled to print, kept apart from the decision that
//! selects one. Each states what the hardware acknowledged and nothing more:
//! an operator reading the boot log should be able to tell whether this
//! machine confines DMA without reading the source.

use super::message::reason;
use crate::arch::x86_64::acpi::parser::other::foreign_segment_units;
use crate::arch::x86_64::iommu::globals::page_levels;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::fault::drain_faults;
use crate::arch::x86_64::iommu::unit::probe::unit_count;
use crate::sys::serial::{self, Line};

pub(super) fn not_built_in() {
    serial::println(b"[VT-D] enforcement not built in; DMA is unrestricted");
}

/// Identity mapping does not confine a device that was enumerated; what it
/// buys is that anything absent from the enumeration is denied. Every segment
/// 0 unit is programmed; units on other segments are not, and an operator has
/// to know that before trusting the machine.
pub(super) fn enabled(assigned: usize) {
    let mut line = Line::new();
    line.str(b"[VT-D] translation enabled, levels=").hex(page_levels().unwrap_or(0) as u64);
    line.str(b" devices=").hex(assigned as u64).end();
    serial::println(b"[VT-D] enumerated devices identity mapped; others denied");
    Line::new().str(b"[VT-D] units programmed=").hex(unit_count() as u64).end();
    let foreign = foreign_segment_units();
    if foreign > 0 {
        let mut line = Line::new();
        line.str(b"[VT-D] WARNING units on other segments=").hex(foreign as u64);
        line.str(b"; not programmed, devices behind them are unrestricted").end();
    }

    // Anything recorded before this point came from firmware's own tables and
    // describes a machine we no longer run.
    drain_faults();
}

pub(super) fn failed(e: VtdError) {
    let mut line = Line::new();
    line.str(b"[VT-D] bring-up failed (").str(reason(e));
    line.str(b"); DMA is unrestricted").end();
}
