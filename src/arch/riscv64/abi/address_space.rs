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

use crate::memory::addr::PhysAddr;

use super::super::mmu::{make_satp, write_satp, KERNEL_MMU_MODE};

/* Encodes the mode the tables were built for, not the live one: a switch
made before paging is on, or with a satp that reads as Unknown, would
otherwise write Bare and run the target with translation off. */
#[inline(always)]
pub(super) unsafe fn switch(root: PhysAddr) {
    let ppn = (root.as_u64() as usize) >> 12;
    let Some(satp) = make_satp(KERNEL_MMU_MODE, 0, ppn) else {
        return;
    };
    write_satp(satp);
}
