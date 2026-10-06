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

//! The modules the loader left in loader memory, as byte slices.

use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;
use crate::syscall::microkernel::install_source_modules::find;

/// The module of `kind`, or `None` when the loader left none, it is longer
/// than `max`, or either of its ends is outside the directmap.
pub(crate) fn module_bytes(kind: u32, max: u64) -> Option<&'static [u8]> {
    let m = find(u64::from(kind))?;
    if m.size == 0 || m.size > max {
        return None;
    }
    let last = m.base.checked_add(m.size - 1)?;
    phys_to_virt(PhysAddr::new(last))?;
    let virt = phys_to_virt(PhysAddr::new(m.base))?;
    /*
     * SAFETY: eK@nonos.systems - the loader left this region in loader memory,
     * which the kernel never frees or writes, and both of its ends were just
     * found under the directmap.
     */
    Some(unsafe { core::slice::from_raw_parts(virt.as_u64() as *const u8, m.size as usize) })
}
