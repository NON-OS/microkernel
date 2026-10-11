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

//! One DMA region as the CPU and the device see it.

/// One DMA region: `va` is where the CPU reaches it, `bus` the address the
/// controller is given, `len` its size in bytes. The region must stay mapped
/// while the controller can still reach it; the owner (`platform`) makes
/// sure of that.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DmaBuf {
    pub va: u64,
    pub bus: u64,
    pub len: usize,
}

impl DmaBuf {
    /// Copy `src` into the region at `off`. False, and nothing written, when
    /// it does not fit.
    pub fn put(&self, off: usize, src: &[u8]) -> bool {
        match off.checked_add(src.len()) {
            Some(end) if end <= self.len => {}
            _ => return false,
        }
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), (self.va as *mut u8).add(off), src.len());
        }
        // The controller reads the region only after the command register
        // write that follows; keep these stores ahead of it.
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        true
    }

    /// Copy the region from `off` into `dst`. False, and nothing copied, when
    /// it does not fit.
    pub fn get(&self, off: usize, dst: &mut [u8]) -> bool {
        match off.checked_add(dst.len()) {
            Some(end) if end <= self.len => {}
            _ => return false,
        }
        // The controller's writes landed before it posted Transfer Complete,
        // which was read before this; keep these loads after that read.
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        unsafe {
            core::ptr::copy_nonoverlapping(
                (self.va as *const u8).add(off),
                dst.as_mut_ptr(),
                dst.len(),
            );
        }
        true
    }
}
