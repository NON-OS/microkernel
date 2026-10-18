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

//! `MkIrqBind` core. Two paths share the same syscall:
//!
//!   * INTx (default, `flags == 0`) — programs the IO-APIC, returns
//!     a single grant. Behaviour is bit-identical to the pre-MSI-X
//!     version of the broker.
//!   * MSI-X (`flags & BIND_MSIX != 0`) — kernel walks the
//!     capability list, validates the table BAR against the
//!     claimed device, allocates `vector_count` contiguous broker
//!     vectors, programs that many MSI-X table entries with the
//!     LAPIC redirect, enables MSI-X, then unmasks each entry. The
//!     capsule never sees the table address and never writes to
//!     it; it only receives the base grant id and base vector.
//!
//! All hardware-touching steps go through the `MsixOps` indirection.

use super::super::types::{IrqBindError, IrqBindRequest, IrqBindResult, BIND_MSI, BIND_MSIX};
use super::intx::bind_intx;
use super::msi::bind_msi;
use super::msix::bind_msix;
use crate::hardware::broker::claim;

pub fn bind(pid: u32, req: IrqBindRequest) -> Result<IrqBindResult, IrqBindError> {
    let claim = claim::lookup(req.device_id).ok_or(IrqBindError::NotClaimed)?;
    if claim.pid != pid {
        return Err(IrqBindError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(IrqBindError::StaleEpoch);
    }

    let bound = if req.flags & BIND_MSIX != 0 {
        bind_msix(pid, req, claim.epoch)
    } else if req.flags & BIND_MSI != 0 {
        bind_msi(pid, req, claim.epoch)
    } else {
        bind_intx(pid, req, claim.epoch)
    }?;
    // Entered before the claim's release took its grants, or undone here
    // (K10): a release since the check above leaves no binding behind.
    if claim::record_while_held(pid, req.device_id, claim.epoch, || ()).is_none() {
        crate::hardware::broker::irq::release_for_device(pid, req.device_id);
        return Err(IrqBindError::StaleEpoch);
    }
    Ok(bound)
}
