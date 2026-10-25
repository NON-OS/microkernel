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

//! Taking the handoff while the MMU is off and physical addresses are what the
//! CPU reads: copied into the kernel's own memory, which moves with it to the
//! high half, where the loader's page would not be mapped.

use core::mem::size_of;

use spin::Once;

use super::place::placed;
use crate::arch::aarch64::boot::info::BootInfo;
use crate::boot::handoff::{BootHandoffV1, HANDOFF_MAGIC, HANDOFF_VERSION};

pub(super) static COPY: Once<BootHandoffV1> = Once::new();

/// Copy the handoff at `phys`, or do nothing for zero. A non-zero x1 that is
/// not a whole, current NONOS handoff refuses the boot: a loader said it
/// verified this kernel, and what it handed over cannot be read as that.
pub(in crate::arch::aarch64::boot) fn take(
    phys: u64,
    info: &BootInfo,
) -> Result<(), &'static [u8]> {
    if phys == 0 {
        return Ok(());
    }
    let len = size_of::<BootHandoffV1>() as u64;
    let regions = info.memory_map().iter().map(|r| (r.base, r.size));
    if !placed(regions, phys, len) {
        return Err(b"the handoff lies outside RAM or is misaligned");
    }
    let Some(at) = crate::arch::aarch64::mmu::pointer::ram::<BootHandoffV1>(phys) else {
        return Err(b"the handoff lies past the direct map");
    };
    // SAFETY: the MMU is off, so `at` is `phys` itself (`ram`), and
    // `[phys, phys + len)` is RAM the tree names (`placed`). The loader
    // cleaned it to the point of coherency before the jump (arm64 boot
    // protocol), so the uncached read sees what it wrote.
    let handoff = unsafe { core::ptr::read_volatile(at) };
    if handoff.magic != HANDOFF_MAGIC {
        return Err(b"x1 holds no NONOS loader handoff");
    }
    if handoff.version != HANDOFF_VERSION || u64::from(handoff.size) != len {
        return Err(b"the handoff is another version or size than this kernel's");
    }
    COPY.call_once(|| handoff);
    Ok(())
}
