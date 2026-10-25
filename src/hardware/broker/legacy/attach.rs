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

//! Puts a function's fixed windows into its broker record as I/O BARs
//! flagged fixed. The PIO grant then hands them out like any BAR, and only
//! to the function's claimant. The record's interrupt fields stay the
//! function's own INTx; the compatibility lines are the specification's and
//! the PIO broker checks a bind against them.

use super::compat::{ide, Ports};
use super::dispi::dispi;
use crate::hardware::broker::device::{BarKind, BAR_FLAG_FIXED};
use crate::hardware::broker::{Bar, DeviceRecord};

pub(in crate::hardware::broker) fn attach(rec: &mut DeviceRecord) {
    let has_io_bar = rec.bars.iter().any(|b| b.kind == BarKind::Pio as u8);
    // A compatibility channel ignores whatever its BARs hold, so the fixed
    // window replaces the slot rather than looking for a free one.
    let fixed = ide(rec.pci_class, rec.pci_subclass, rec.pci_progif);
    for (slot, ports) in fixed.bars.iter().enumerate() {
        if let Some(p) = ports {
            place(rec, slot, *p);
        }
    }
    if let Some(p) = dispi(rec.vendor, rec.device, has_io_bar) {
        if let Some(slot) = rec.bars.iter().position(|b| b.kind == BarKind::None as u8) {
            place(rec, slot, p);
        }
    }
}

fn place(rec: &mut DeviceRecord, slot: usize, p: Ports) {
    rec.bars[slot] = Bar {
        base: p.base as u64,
        size: p.count as u64,
        kind: BarKind::Pio as u8,
        flags: BAR_FLAG_FIXED,
        aux: 0,
        _pad: [0; 2],
    };
    rec.bar_count = rec.bar_count.max(slot as u8 + 1);
}
