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

//! Interrupt remapping at boot: one table, every unit pointed at it, and a
//! line that says whether it took. A unit without ECAP.IR, or without a
//! live invalidation queue, leaves it off on the whole machine, since a
//! device's handle must mean the same entry behind every unit.

use core::sync::atomic::Ordering;

use super::enable::enable_on;
use super::table::{ACTIVE, TABLE};
use crate::arch::x86_64::iommu::regs::{cap, offsets};
use crate::arch::x86_64::iommu::tables::frame::allocate_table;
use crate::arch::x86_64::iommu::unit::queue;
use crate::arch::x86_64::iommu::unit::report::units;
use crate::sys::serial::{self, Line};

pub fn init() {
    let all = units();
    if all.is_empty() {
        return;
    }
    if !all.iter().all(|u| cap::interrupt_remapping(u.ecap)) {
        serial::println(b"[VT-D] IOMMU interrupt remapping off: a unit lacks ECAP.IR");
        return;
    }
    if !all.iter().all(|u| queue::is_live(&u.unit)) {
        serial::println(b"[VT-D] IOMMU interrupt remapping off: a unit has no live queue");
        return;
    }
    let mut table = TABLE.lock();
    if table.phys == 0 {
        match allocate_table() {
            Ok(phys) => table.phys = phys,
            Err(_) => {
                serial::println(b"[VT-D] IOMMU interrupt remapping off: no frame for the table");
                return;
            }
        }
    }
    // xAPIC-format entries (EIME clear): CFI is honoured only then, and the
    // kernel's MSI messages are compatibility format until each driver moves.
    let irta = offsets::irta_value(table.phys, false);
    for info in all {
        // SAFETY: eK@nonos.systems - the table page is allocated above and
        // never freed, and every unit's queue was checked live.
        if unsafe { enable_on(&info.unit, irta) }.is_err() {
            let mut line = Line::new();
            line.str(b"[VT-D] IOMMU interrupt remapping off: unit base=").hex(info.unit.base_pa());
            line.str(b" did not acknowledge").end();
            return;
        }
    }
    ACTIVE.store(true, Ordering::Release);
    let mut line = Line::new();
    line.str(b"[VT-D] IOMMU interrupt remapping on, entries=").dec(offsets::IRTE_ENTRIES as u64);
    line.str(b", compatibility format interrupts passed (CFI)").end();
}
