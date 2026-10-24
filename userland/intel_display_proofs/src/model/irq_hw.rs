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

//! The display engine's interrupt identity registers in the model:
//! DE_PIPE_IIR of each pipe clears the bits written as one.

use super::gen9::Gen9;

impl Gen9 {
    pub(super) fn iir_write(&mut self, off: u32, v: u32, old: u32) {
        if (0..4).any(|p| off == 0x4_4408 + 0x10 * p) {
            self.regs.insert(off, old & !v);
        }
    }
}
