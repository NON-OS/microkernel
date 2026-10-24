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

use super::super::boot::info::BootInfo;
use super::{control, state, tables_build, ttbr};

pub fn init_mmu(boot_info: &BootInfo) {
    control::configure_mair();
    control::configure_tcr();
    // SAFETY: the boot CPU, once, with translation still off.
    unsafe {
        tables_build::build(boot_info);
        ttbr::set_ttbr1(state::l0_addr());
        ttbr::set_ttbr0(state::l0_addr(), 0);
    }
    if !super::kernel_va::high_mapped() {
        crate::sys::serial::println(b"[MMU] high alias incomplete: the kernel stays one to one");
    }
    control::enable_mmu();
}
