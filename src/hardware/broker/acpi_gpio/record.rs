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

use crate::arch::x86_64::acpi::aml::GpioController;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, BusKind, DeviceRecord};

/// The bar that carries the controller's `_HID`, past every window.
const HID_BAR: usize = 5;

/// Build an ACPI-bus DeviceRecord for a GPIO controller: bars 0.. are its
/// community windows in Linux barno order, so a driver claims it and maps
/// the window a pin's pad lives in through `MkMmioMap`. Bar 5 (kind None,
/// never mapped, past `bar_count`) carries the eight `_HID` bytes as a
/// little-endian `base`, which picks the pad layout in nonos_pinctrl. The
/// firmware `_UID`, which a touchpad record refers to, rides in `device`.
pub(super) fn device_record(ctl: &GpioController) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    let count = ctl.window_count.min(HID_BAR);
    for (bar, &(base, size)) in bars.iter_mut().zip(&ctl.windows[..count]) {
        *bar = Bar { base, size, kind: BarKind::Mmio as u8, flags: 0, aux: 0, _pad: [0; 2] };
    }
    bars[HID_BAR] = Bar {
        base: u64::from_le_bytes(ctl.hid),
        size: 0,
        kind: BarKind::None as u8,
        flags: 0,
        aux: 0,
        _pad: [0; 2],
    };
    DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::GPIO_CTRL,
        vendor: 0,
        device: (ctl.uid & 0xFFFF) as u16,
        bar_count: count as u8,
        bars,
        ..DeviceRecord::empty()
    }
}
