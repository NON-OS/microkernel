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

//! The device's side of the DMA grants: a device model reads and writes
//! the driver's memory at the device addresses it was told, as a device
//! does. Both are false, touching nothing, outside every grant.

use crate::dma::locate;

pub fn dma_read(phys: u64, out: &mut [u8]) -> bool {
    let Some(at) = locate(phys, out.len()) else {
        return false;
    };
    // SAFETY: `locate` found all of `out.len()` bytes inside one live grant,
    // which is never freed; the driver reaches it only through raw-pointer
    // views, never a reference, and not during this call.
    unsafe { std::ptr::copy_nonoverlapping(at, out.as_mut_ptr(), out.len()) };
    true
}

pub fn dma_write(phys: u64, bytes: &[u8]) -> bool {
    let Some(at) = locate(phys, bytes.len()) else {
        return false;
    };
    // SAFETY: as in `dma_read`; the bytes are the model's own.
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), at, bytes.len()) };
    true
}
