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

//! A client's surface lent to the display engine: attached here (its owner
//! must have named this driver a receiver), then imported read only into
//! the device's IOMMU domain (`MkDmaImportSurface`). Given back in the
//! other order: the import first, which the kernel answers only once the
//! device can no longer read it, then the attach.

use nonos_gfx_wire::errno::{ENODEV, ENOMEM, EPERM};
use nonos_gfx_wire::Errno;
use nonos_intel_display::display::Lent;
use nonos_libc::{
    mk_dma_import_surface, mk_dma_unimport_surface, mk_surface_attach, mk_surface_release,
    DmaImportOut, SurfaceDescriptor,
};

/// The kernel's ENODEV: no IOMMU confines the device.
const K_ENODEV: i64 = -19;

pub fn lend(device_id: u64, epoch: u64, handle: u64) -> Result<Lent, Errno> {
    let mut desc = SurfaceDescriptor::default();
    if mk_surface_attach(handle, &mut desc) <= 0 {
        return Err(EPERM);
    }
    let mut out = DmaImportOut::default();
    let rc = mk_dma_import_surface(device_id, epoch, handle, &mut out);
    if rc < 0 {
        let _ = mk_surface_release(handle);
        return Err(match rc {
            K_ENODEV => ENODEV,
            -1 => EPERM,
            _ => ENOMEM,
        });
    }
    Ok(Lent { handle, iova: out.device_addr, len: out.length, id: out.import_id })
}

pub fn give_back(lent: Lent) {
    let _ = mk_dma_unimport_surface(lent.id);
    let _ = mk_surface_release(lent.handle);
}
