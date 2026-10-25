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

//! How a device's own description says its interrupt line is driven (L43).
//! An ACPI platform device's record carries its `_CRS` trigger and polarity
//! (F 51); a PCI function's does not, and its line is sensed as the MADT and
//! the ISA or PCI default say.

use crate::hardware::broker::{
    BusKind, DeviceRecord, DEVICE_FLAG_IRQ_ACTIVE_LOW, DEVICE_FLAG_IRQ_EDGE,
};

/// (level, active-low), when the record describes its interrupt.
pub(super) fn described(record: &DeviceRecord) -> Option<(bool, bool)> {
    if record.bus_kind != BusKind::Acpi as u8 || record.irq_pin != 1 {
        return None;
    }
    let level = record.flags & DEVICE_FLAG_IRQ_EDGE == 0;
    Some((level, record.flags & DEVICE_FLAG_IRQ_ACTIVE_LOW != 0))
}
