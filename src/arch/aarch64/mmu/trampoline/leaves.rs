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

//! The trampoline's two leaves' attributes, apart from the tables so the
//! proofs walk what the kernel maps rather than a copy of it.

use crate::arch::aarch64::mmu::attributes::PageAttributes;

/// The code page's and the data page's descriptor bits. The code is the
/// kernel's own text, so it is mapped as the kernel maps text: read-only,
/// run at EL1 only, guarded where the kernel's text is. The data page is
/// read-only and runs nowhere.
pub(crate) fn bits(guarded: bool) -> [u64; 2] {
    [
        PageAttributes::kernel_code().with_guard(guarded).to_descriptor_bits(),
        PageAttributes::kernel_rodata().to_descriptor_bits(),
    ]
}
