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

//! Pure validation routines used by the `MkIrqBind` MSI-X path.
//! Everything in this module is a function over plain inputs — no
//! globals, no MMIO, no allocation. The bind path runs the
//! validators after looking up the kernel-side state.
//!
//! Errors are returned in a fixed priority order so a capsule
//! gets a deterministic explanation for a malformed request:
//!
//!   1. `UnsupportedFlags` — flag bits the kernel does not know.
//!   2. `BadVectorCount`   — count is zero, larger than the broker
//!                           pool, or larger than the device's MSI-X
//!                           table.
//!   3. `NoDeviceHandle`   — broker has no PCI side-table entry for
//!                           this `device_id` (e.g. platform device).
//!   4. `NoMsixCap`        — device does not advertise an MSI-X cap.
//!   5. `BadMsixBar`       — table or PBA BAR is out of range, not
//!                           memory-mapped, or not present.
//!   6. `NotDeviceIrq`     — MSI-X mode requires `irq_source == 0`.
//!   7. `AlreadyBound`:    the device already has an MSI or MSI-X
//!                           grant; MSI-X bind is all-or-nothing per
//!                           device.

use super::super::types::{IrqBindError, IrqBindRequest, BIND_MSIX, FLAGS_KNOWN};
use super::msix_view::MsixHandleView;

/// Validate the MSI-X branch of `MkIrqBind`; `Ok` lets the caller allocate
/// slots and program the device. `pool_capacity` is the broker pool size,
/// passed in so the validator stays free of globals; `handle` is `None` when
/// the broker has no PCI side-table entry for this device id.
pub(in super::super) fn validate_msix_request(
    req: &IrqBindRequest,
    pool_capacity: usize,
    handle: Option<&MsixHandleView>,
    has_message_grant: bool,
) -> Result<(), IrqBindError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(IrqBindError::UnsupportedFlags);
    }
    if req.flags != BIND_MSIX {
        // An INTx request, or MSI asked together with MSI-X; a function
        // with both enabled is undefined (PCI Local Bus 3.0, 6.8).
        return Err(IrqBindError::UnsupportedFlags);
    }
    let n = req.vector_count as usize;
    if n == 0 || n > pool_capacity {
        return Err(IrqBindError::BadVectorCount);
    }
    handle.ok_or(IrqBindError::NoDeviceHandle)?.check(n)?;
    if req.irq_source != 0 {
        return Err(IrqBindError::NotDeviceIrq);
    }
    if has_message_grant {
        return Err(IrqBindError::AlreadyBound);
    }
    Ok(())
}
