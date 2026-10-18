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

//! Reset (waited for by the clock), ACKNOWLEDGE, DRIVER, then the features:
//! VIRGL and EDID when offered, VERSION_1, and ACCESS_PLATFORM whenever it
//! is offered, so a device behind an IOMMU sends its DMA through it to the
//! grants' addresses; FEATURES_OK, read back. A device without VERSION_1,
//! or one that clears FEATURES_OK, is left FAILED. Whether the claim is
//! confined is the kernel's to enforce at claim time; the driver does not
//! guess. The negotiation used to be the driver's own, and its reset never
//! waited for the device to read back zero.

use nonos_virtio::common::{accept, start_until};
use nonos_virtio::{Mmio, VirtioError};

use crate::clock::expired_after;
use crate::constants::{RESET_MS, VIRTIO_GPU_F_EDID, VIRTIO_GPU_F_VIRGL};

/// The features in force.
pub fn negotiate(common: &Mmio) -> Result<u64, &'static str> {
    start_until(common, expired_after(RESET_MS)).map_err(VirtioError::message)?;
    let handled = u64::from(VIRTIO_GPU_F_VIRGL | VIRTIO_GPU_F_EDID);
    accept(common, handled).map_err(VirtioError::message)
}
