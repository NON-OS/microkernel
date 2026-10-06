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

//! The heap's region, mapped in pieces. `MkMmap` maps at most a gigabyte a
//! call, so a larger heap is several calls whose ranges must meet end to end.
//! The kernel hands each process fresh ranges from a cursor that only moves
//! up, so before anything is freed they do; a piece that does not meet the
//! last, or does not map, undoes every piece before it. One piece is exactly
//! the single map a heap of a gigabyte or less always was.

use crate::mem::{mk_mmap, mk_munmap};

/// The kernel's `MAX_MMAP_SIZE`.
const PIECE: usize = 1 << 30;
/// Read and write; only `prot` selects the page flags.
const PROT: i32 = 0x1 | 0x2;
/// Private and anonymous, which the microkernel ignores.
const FLAGS: i32 = 0x02 | 0x20;
const USERSPACE_MAX: usize = 0x0000_7FFF_FFFF_FFFF;

/// The base of `bytes` of fresh, contiguous, writable memory. Up to `PIECE`
/// this is exactly the one call, and the one check, it always was.
pub(super) fn map(bytes: usize) -> Option<*mut u8> {
    let base = piece(bytes.min(PIECE))?;
    let mut done = bytes.min(PIECE);
    while done < bytes {
        let len = (bytes - done).min(PIECE);
        match piece(len) {
            Some(p) if p as usize == base as usize + done => done += len,
            other => {
                if let Some(p) = other {
                    mk_munmap(p, len);
                }
                undo(base, done);
                return None;
            }
        }
    }
    Some(base)
}

fn piece(len: usize) -> Option<*mut u8> {
    let p = mk_mmap(core::ptr::null_mut(), len, PROT, FLAGS, -1, 0);
    (!p.is_null() && (p as i64) >= 0 && p as usize <= USERSPACE_MAX).then_some(p)
}

/* Unmap the pieces of `done` bytes from `base`, each as it was mapped. */
fn undo(base: *mut u8, done: usize) {
    let mut at = 0;
    while at < done {
        let len = (done - at).min(PIECE);
        mk_munmap(base.wrapping_add(at), len);
        at += len;
    }
}
