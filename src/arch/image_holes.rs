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

//! Pages inside the kernel image that are meant to be missing.
//!
//! Anything walking the image and expecting every page to be mapped needs
//! this, or it reports the stack guards as holes. Where those guards are is a
//! fact about the architecture's stack layout, so the answer comes from the
//! architecture rather than from a list the memory layer would have to keep
//! in step by hand.

/// True when nothing should map `va`, and an absent mapping there is the
/// mechanism working rather than damage.
pub fn deliberately_unmapped(va: u64) -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::gdt::is_stack_guard(va)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = va;
        false
    }
}
