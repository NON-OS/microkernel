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

//! The image's pointers rewritten for the address it is about to run at, its
//! high alias, while it still runs where it was loaded.
//!
//! The MMU is on by then and text and read-only data are mapped read only, so
//! every place must lie in the writable part, from `writable` bytes into the
//! image; the link places every pointer there, and `check_kernel_elf.py`
//! holds it to that. As in `apply`, every check runs before the first write:
//! a refused rebase leaves the image as it was, running where it is.

use super::apply::place;
use super::rela::{Placement, Refused, Rela, LOAD_ALIGN};

/// Write each pointer as `run_base` makes it, through `at.load_base`, where
/// the image is reachable now. Returns how many were written.
///
/// # Safety
///
/// `[at.load_base + writable, at.load_base + at.len)` must be the loaded
/// image's writable part, and nothing may read a pointer in it until the
/// image runs at `run_base`.
pub(crate) unsafe fn rebase(
    relas: &[Rela],
    at: Placement,
    run_base: u64,
    writable: u64,
) -> Result<usize, Refused> {
    let bias = run_base.wrapping_sub(at.link_base);
    if !bias.is_multiple_of(LOAD_ALIGN) {
        return Err(Refused::LoadMisaligned);
    }
    for r in relas {
        place(r, at)?;
        if r.offset.wrapping_sub(at.link_base) < writable {
            return Err(Refused::ReadOnlyPlace);
        }
    }
    for r in relas {
        let at_place = at.load_base.wrapping_add(r.offset.wrapping_sub(at.link_base));
        // The image's own address, as it runs, not a physical one: counted
        // by run-static-checks.sh.
        #[cfg_attr(target_os = "none", allow(fuzzy_provenance_casts))]
        let word = at_place as *mut u64;
        // SAFETY: `place` and the check above put these 8 bytes inside the
        // image's writable part, aligned; the caller keeps them unread.
        unsafe { core::ptr::write_volatile(word, r.addend.wrapping_add(bias)) };
    }
    Ok(relas.len())
}
