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

//! What other capsules hold: their claimed devices' BARs and functions, and
//! the ACPI windows another pid was granted.

use super::super::super::claim;
use super::super::super::device::{BAR_KIND_MMIO, BAR_KIND_PIO};
use super::super::super::{pci_index, table};
use super::records::{other_holds, Space};

/// A claimed device's whole BAR is its driver's, mapped yet or not.
fn bar_held(pid: u32, kind: u8, base: u64, end: u64) -> bool {
    table::list().iter().any(|r| {
        let theirs = claim::lookup(r.device_id).is_some_and(|c| c.pid != pid);
        let count = (r.bar_count as usize).min(r.bars.len());
        theirs
            && r.bars[..count].iter().any(|b| {
                b.kind == kind
                    && b.size != 0
                    && b.base < end
                    && base < b.base.saturating_add(b.size)
            })
    })
}

pub(in super::super) fn mmio(pid: u32, base: u64, end: u64) -> bool {
    bar_held(pid, BAR_KIND_MMIO, base, end) || other_holds(pid, Space::Memory, base, end)
}

pub(in super::super) fn ports(pid: u32, base: u64, end: u64) -> bool {
    bar_held(pid, BAR_KIND_PIO, base, end) || other_holds(pid, Space::Io, base, end)
}

/// Only segment 0 is enumerated, so only it can be claimed.
pub(in super::super) fn function(pid: u32, bdf: (u16, u8, u8, u8)) -> bool {
    let (seg, bus, dev, func) = bdf;
    pci_index::all().iter().any(|h| {
        let a = h.address;
        let same = seg == 0 && a.segment == 0 && (a.bus, a.device, a.function) == (bus, dev, func);
        same && claim::lookup(h.device_id).is_some_and(|c| c.pid != pid)
    })
}
