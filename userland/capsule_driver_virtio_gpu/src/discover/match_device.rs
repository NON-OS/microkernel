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

use nonos_libc::{DeviceRecord, BUS_KIND_PCI};

use crate::constants::{VIRTIO_GPU_MODERN, VIRTIO_VENDOR_ID};

/// Identity only. The driver binds no interrupt, so a device whose legacy
/// line firmware left unrouted (irq_line 0xFF) is as usable as any other.
/// Only the modern id: virtio-gpu has no transitional form, and the legacy
/// path kept for 0x1010 could never be reached.
pub fn is_match(r: &DeviceRecord) -> bool {
    r.vendor == VIRTIO_VENDOR_ID && r.bus_kind == BUS_KIND_PCI && r.device == VIRTIO_GPU_MODERN
}
