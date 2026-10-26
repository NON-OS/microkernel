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

use crate::arch::aarch64::gic::msi::{self, Vector, MAX_VECTORS};
use crate::hardware::broker::irq::types::{IrqBindError, IrqBindRequest, IrqBindResult, BIND_MSIX};
use crate::hardware::broker::pci_index::{self, msi_of};

use super::super::validate::{validate_msi_request, validate_msix_request};
use super::handle_view::handle_view;
use super::record::{MessageBind, BINDS};

/// Bind the claimed function's MSI or MSI-X. The checks are x86's, in its
/// order; then the function's requester id must reach a controller the
/// kernel drives through the tree's `msi-map`, or there is nothing to bind.
pub(in super::super) fn bind(
    pid: u32,
    req: IrqBindRequest,
    epoch: u64,
) -> Result<IrqBindResult, IrqBindError> {
    let handle = pci_index::lookup(req.device_id);
    let mut binds = BINDS.lock();
    let held = binds.iter().flatten().any(|b| b.device_id == req.device_id);
    let msix = req.flags & BIND_MSIX != 0;
    if msix {
        let view = handle.as_ref().map(handle_view);
        validate_msix_request(&req, MAX_VECTORS, view.as_ref(), held)?;
    } else {
        validate_msi_request(&req, handle.as_ref().map(|h| msi_of(h).is_some()), held)?;
    }
    let handle = handle.ok_or(IrqBindError::NoDeviceHandle)?;
    let slot = binds.iter().position(Option::is_none).ok_or(IrqBindError::NoVector)?;
    let a = handle.address;
    let rid = u16::from(a.bus) << 8 | u16::from(a.device) << 3 | u16::from(a.function);
    let msi_id = msi::device_id(rid).ok_or(IrqBindError::PlatformError)?;
    let mut b = MessageBind {
        base: 0,
        pid,
        device_id: req.device_id,
        msi_id,
        msix,
        vectors: [Vector::default(); MAX_VECTORS],
        count: if msix { req.vector_count as usize } else { 1 },
        live: 0,
        doorbell: 0,
        confined: false,
    };
    if let Err(e) = super::open::open(&mut b, &handle, epoch) {
        super::close::close_all(&mut b, Some(&handle));
        crate::log::info!("[IRQ] {} message bind failed: {:?}", a, e);
        return Err(e);
    }
    // Entered only while its claim stands, under the lock its drain takes.
    if !crate::hardware::broker::claim::live(pid, req.device_id, epoch) {
        super::close::close_all(&mut b, Some(&handle));
        return Err(IrqBindError::NotClaimed);
    }
    binds[slot] = Some(b);
    crate::log::info!("[IRQ] {} {} vectors from intid {}", a, b.count, b.vectors[0].intid);
    Ok(IrqBindResult { grant_id: b.base, vector: 0, grants: b.count as u32 })
}
