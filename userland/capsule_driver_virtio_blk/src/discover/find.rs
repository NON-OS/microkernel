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

use nonos_libc::{mk_device_list, DeviceRecord};

use super::found::Found;

/// The device list holds ACPI and fabricated records beside PCI functions;
/// at 32 a machine with more stopped short of the device behind a root port.
const MAX_DEVICES: usize = 128;

/// The first virtio-blk function with a register BAR. Its interrupt is not
/// asked about here: one with no INTx pin, or a line the firmware left at
/// 0xFF (common under UEFI), is driven by MSI-X (`setup::irq`). Filtering
/// on the pin and line here made the MSI-X path unreachable for exactly
/// the functions that need it.
pub fn find_virtio_blk() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = usize::try_from(n).ok()?.min(MAX_DEVICES);
    buf.iter().take(count).find_map(Found::of)
}
