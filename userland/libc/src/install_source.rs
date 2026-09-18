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

//! The image this machine booted, as the kernel holds it: the bootloader
//! and the attested kernel image the bootloader left in memory. The
//! installer writes these to a disk, so an install is the verified image
//! and nothing read back from a medium.

use crate::syscall::{call_raw, N_MK_INSTALL_SOURCE};

pub const INSTALL_SOURCE_LOADER_IMAGE: u64 = 1;
pub const INSTALL_SOURCE_KERNEL_IMAGE: u64 = 2;

/// Copy up to `out.len()` bytes of image `kind` starting at `offset`. Returns
/// the bytes written, zero at the end of the image, or a negative errno. The
/// kernel caps one call at a mebibyte, so read in a loop until zero.
pub fn mk_install_source(kind: u64, offset: u64, out: &mut [u8]) -> i64 {
    call_raw(N_MK_INSTALL_SOURCE, [kind, offset, out.as_mut_ptr() as u64, out.len() as u64, 0, 0])
}

/// The size of image `kind` in bytes, or `None` when the kernel holds no
/// such image. Found by probing one byte at a time in a binary search over
/// the offset, so a caller can size its memory before reading a byte of the
/// image: a copy of it is a hundred megabytes and the default capsule heap
/// is sixteen.
pub fn mk_install_source_size(kind: u64) -> Option<u64> {
    let mut probe = [0u8; 1];
    if mk_install_source(kind, 0, &mut probe) < 0 {
        return None;
    }
    let (mut lo, mut hi) = (0u64, 1u64 << 32);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if mk_install_source(kind, mid, &mut probe) == 1 {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    Some(lo)
}
