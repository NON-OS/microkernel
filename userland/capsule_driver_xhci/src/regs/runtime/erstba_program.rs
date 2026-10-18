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
use crate::constants::ERSTBA_LO;
use crate::regs::mmio_write64;
pub fn erstba_program(intr_base: u64, erst_phys: u64) {
    // SAFETY: `intr_base` is interrupter 0's register set, 32 bytes into the runtime page
    // `read_layout` checked lies inside the mapped window.
    unsafe { mmio_write64(intr_base + ERSTBA_LO, erst_phys & !0x3F) };
}
