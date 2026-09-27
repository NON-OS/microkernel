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

use crate::drivers::pci::config::ConfigSpace;

const VIRTIO_VENDOR: u16 = 0x1AF4;

/*
 * A virtio device translates its DMA through the IOMMU only once the driver
 * negotiates VIRTIO_F_ACCESS_PLATFORM, and no driver here does (the GPU runs
 * the legacy interface, which cannot). Such a device takes the address it is
 * given as physical. Confining it would hand it IOVAs it writes through as
 * physical memory, so it is left unconfined and said to be.
 */
pub(super) fn bypasses_translation(device_id: u64) -> bool {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return false;
    };
    matches!(ConfigSpace::new(handle.address).read16(0), Ok(VIRTIO_VENDOR))
}
