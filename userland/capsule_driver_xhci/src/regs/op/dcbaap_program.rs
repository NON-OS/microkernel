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
use crate::constants::DCBAAP_LO;
use crate::regs::mmio_write64;
pub fn dcbaap_program(op_base: u64, dcbaa_phys: u64) {
    // SAFETY: `op_base` is the operational base, which `read_layout` checked lies with the page
    // after it inside the mapped window.
    unsafe { mmio_write64(op_base + DCBAAP_LO, dcbaa_phys & !0x3F) };
}
