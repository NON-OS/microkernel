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

//! The boot check that a core coming back from a powerdown state can run
//! its resume entry one to one under the root it turns its MMU on with.
//! Made once; until it passes, and if it fails, no powerdown state is asked
//! for.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::arch::aarch64::mmu::one_to_one::executable_one_to_one;
use crate::arch::aarch64::mmu::pointer::ram;
use crate::arch::aarch64::mmu::{image_pa, published_root, runs_high, PageTable};

extern "C" {
    fn _aarch64_resume_start();
    fn _aarch64_resume_start_end();
    fn aarch64_resume_high();
    fn aarch64_resume_high_end();
}

static VERIFIED: AtomicBool = AtomicBool::new(false);

pub(super) fn verified() -> bool {
    VERIFIED.load(Ordering::Acquire)
}

/// The physical span from one label to another in the image.
fn span(from: unsafe extern "C" fn(), to: unsafe extern "C" fn()) -> (u64, u64) {
    (image_pa(from as u64), image_pa(to as u64))
}

/// Check the resume entry, and the rest of the resume too while the kernel
/// still runs one to one; on success powerdown states may be used. Why not,
/// else.
pub(super) fn verify() -> Result<(), &'static [u8]> {
    let root = published_root().ok_or(&b"no MMU record for a core to come back with"[..])?;
    let reach = |pa: u64| {
        ram::<PageTable>(pa).map(|table| {
            // SAFETY: a table of the published root, read only, in RAM the
            // direct map reaches; nothing frees the boot tables.
            unsafe { &*table }
        })
    };
    let root = reach(root).ok_or(&b"the MMU record's root is past the direct map"[..])?;
    let mut spans = [span(_aarch64_resume_start, _aarch64_resume_start_end), (0, 0)];
    if !runs_high() {
        spans[1] = span(aarch64_resume_high, aarch64_resume_high_end);
    }
    for (start, end) in spans {
        executable_one_to_one(root, start, end, reach)?;
    }
    VERIFIED.store(true, Ordering::Release);
    Ok(())
}
