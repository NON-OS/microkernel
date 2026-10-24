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

//! Which broker records are SD hosts to try: a PCI function of the SD host
//! class, as classify.rs reads it, or an ACPI platform device the firmware
//! names an SD or eMMC host (F 28's records: class 08h subclass 05h,
//! vendor and device 0, its windows as memory BARs in _CRS order). An ACPI
//! host has no PCI configuration space, so it has no Slot Information byte
//! and no command register; its first window is its first slot. A record
//! the broker marks disabled (its window fenced after a departure) is not
//! one.

use super::{classify, Kind};

/// The broker's bus kinds and its disabled flag (libc broker::types).
const BUS_PCI: u8 = 1;
const BUS_ACPI: u8 = 2;
const DISABLED: u32 = 1 << 1;

/// A host to try and whether it is an ACPI one, or None.
pub fn host(
    record: (u8, u32),
    vendor: u16,
    device: u16,
    class: (u8, u8, u8),
) -> Option<(Kind, bool)> {
    let (bus_kind, flags) = record;
    if flags & DISABLED != 0 {
        return None;
    }
    let kind = classify(vendor, device, class.0, class.1, class.2)?;
    match bus_kind {
        BUS_PCI => Some((kind, false)),
        BUS_ACPI => Some((kind, true)),
        _ => None,
    }
}
