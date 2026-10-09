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

//! Pure checks on the MSI branch of `MkIrqBind`, in the MSI-X validator's
//! order. One vector only: multiple message MSI needs a naturally aligned
//! power-of-two block of vectors (PCI Local Bus 3.0, 6.8.1.3), and a capsule
//! that wants more asks for MSI-X.

use super::super::types::{IrqBindError, IrqBindRequest, BIND_MSI};

/// `msi_present` is `None` when the broker has no PCI side-table entry for
/// the device, else whether the function has an MSI capability.
pub(in super::super) fn validate_msi_request(
    req: &IrqBindRequest,
    msi_present: Option<bool>,
    has_message_grant: bool,
) -> Result<(), IrqBindError> {
    if req.flags != BIND_MSI {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.vector_count != 1 {
        return Err(IrqBindError::BadVectorCount);
    }
    if !msi_present.ok_or(IrqBindError::NoDeviceHandle)? {
        return Err(IrqBindError::NoMsiCap);
    }
    if req.irq_source != 0 {
        return Err(IrqBindError::NotDeviceIrq);
    }
    if has_message_grant {
        return Err(IrqBindError::AlreadyBound);
    }
    Ok(())
}
