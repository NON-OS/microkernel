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

//! The SCI as a device the ACPI capsule claims: one record, so the capsule
//! binds the interrupt with MkDeviceClaim and MkIrqBind like any driver, and
//! no other capsule can take the line while it holds it.

use super::kernel::platform::fw;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{BusKind, DeviceRecord};
use crate::hardware::broker::table;
use crate::sys::serial::Line;
use core::sync::atomic::{AtomicU64, AtomicU8, Ordering};

/// "PNP0C08", the ACPI driver's own PnP id, in the encoding the PS/2 records
/// use: vendor 0x0001 for the PNP prefix, the four hex digits as the device.
pub const PNP_VENDOR: u16 = 0x0001;
pub const PNP_DEVICE_ACPI: u16 = 0x0C08;

/// The SCI record's id, then its line, which publishes it; 0 until then.
static SCI_ID: AtomicU64 = AtomicU64::new(0);
static SCI_LINE: AtomicU8 = AtomicU8::new(0);

/// Arm the polled power button (SCI masked, then PWRBTN_EN) unless claimed.
pub fn arm_power_button() {
    let (line, id) = (SCI_LINE.load(Ordering::Acquire), SCI_ID.load(Ordering::Relaxed));
    if line != 0 {
        fw::power_button::arm(line, crate::hardware::broker::claim::lookup(id).is_some());
    }
}

/// Register the SCI the FADT names (ACPI 6.5, 5.2.9, SCI_INT). A
/// hardware-reduced platform has no SCI (4.1) and gets no record; nor does a
/// line past the 8-bit field the broker's records carry.
pub fn register_sci() -> Option<u64> {
    let reduced = fw::parser::with_data(|d| d.fadt.is_some_and(|f| f.is_hw_reduced()));
    let sci = fw::sci_interrupt()?;
    if reduced != Some(false) || sci == 0 {
        return None;
    }
    let Ok(line) = u8::try_from(sci) else {
        Line::new()
            .str(b"[ACPI] sci irq=")
            .dec(u64::from(sci))
            .str(b" past the record's range")
            .end();
        return None;
    };
    let id = table::register_platform_device(DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::OTHER,
        vendor: PNP_VENDOR,
        device: PNP_DEVICE_ACPI,
        irq_line: line,
        irq_pin: 1,
        ..DeviceRecord::empty()
    });
    Line::new().str(b"[ACPI] sci irq=").dec(u64::from(line)).str(b" device=").hex(id).end();
    SCI_ID.store(id, Ordering::Relaxed);
    SCI_LINE.store(line, Ordering::Release);
    Some(id)
}
