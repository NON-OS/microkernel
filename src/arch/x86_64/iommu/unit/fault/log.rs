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

use super::reason::{bdf_text, is_interrupt, reason_text};
use super::record::FaultRecord;
use crate::sys::serial::Line;

/// One line per record, built whole so another CPU's output cannot splice
/// into it. The device reads as lspci prints it; the raw source id follows
/// because the DMAR device scopes key on that form. An interrupt request
/// carries its remapping table index where a DMA fault carries the page.
pub(super) fn log_record(record: &FaultRecord) {
    let mut line = Line::new();
    line.str(b"[VT-D] IOMMU fault ");
    if is_interrupt(record.reason) {
        line.str(b"interrupt dev=").str(&bdf_text(record.source));
        line.str(b" index=").hex(record.address >> 48);
    } else {
        line.str(if record.read { b"read dev=" } else { b"write dev=" });
        line.str(&bdf_text(record.source));
        line.str(b" addr=").hex(record.address & !0xFFF);
    }
    line.str(b" src=").hex(record.source as u64);
    line.str(b" reason=").hex(record.reason as u64).str(b" ");
    line.str(reason_text(record.reason));
    line.end();
}

/// What the budget held back this poll, and every fault since boot.
pub(super) fn log_hidden(hidden: u32) {
    let mut line = Line::new();
    line.str(b"[VT-D] IOMMU faults not shown=").dec(hidden as u64);
    line.str(b" total since boot=").dec(super::count::fault_total());
    line.end();
}

pub(super) fn log_overflow(base: u64) {
    let mut line = Line::new();
    line.str(b"[VT-D] IOMMU fault records overflowed on unit base=").hex(base);
    line.str(b"; some faults were not recorded");
    line.end();
}
