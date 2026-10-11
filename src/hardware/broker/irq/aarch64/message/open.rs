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

use crate::arch::aarch64::gic::msi::{self, MsiError};
use crate::hardware::broker::irq::types::IrqBindError;
use crate::hardware::broker::pci_index::PciHandle;
use crate::memory::iommu::IommuProtection;

use super::quiet::{msi_off, msix_off};
use super::record::MessageBind;
use super::{entries, program};

const PAGE: u64 = 0x1000;

/// Take what the bind needs, in the order it can be given back: the
/// doorbell mapped for the device, vectors from the controller, their slots
/// and handlers, the function programmed, and only then the vectors
/// enabled. A failure leaves `b` saying how far it got, for `close_all`.
pub(super) fn open(
    b: &mut MessageBind,
    handle: &PciHandle,
    epoch: u64,
) -> Result<(), IrqBindError> {
    let bell = msi::doorbell().ok_or(IrqBindError::PlatformError)?;
    let page = bell & !(PAGE - 1);
    let mapped = crate::hardware::broker::confine::map(
        b.pid,
        b.device_id,
        page,
        PAGE,
        IommuProtection::DOORBELL,
    );
    (b.doorbell, b.confined) = mapped.ok_or(IrqBindError::PlatformError)?;
    // MSI and MSI-X both on is undefined; the kind not bound goes off.
    if b.msix {
        msi_off(handle);
    } else {
        msix_off(handle);
    }
    msi::bind(b.msi_id, &mut b.vectors[..b.count]).map_err(|e| match e {
        MsiError::Exhausted | MsiError::NoMemory => IrqBindError::NoVector,
        _ => IrqBindError::PlatformError,
    })?;
    b.live = ((1u64 << b.count) - 1) as u32;
    b.base = entries::open(b, epoch)?;
    program::program(b, handle, b.doorbell + (bell & (PAGE - 1)))?;
    msi::enable(b.msi_id, &b.vectors[..b.count]).map_err(|_| IrqBindError::PlatformError)
}
