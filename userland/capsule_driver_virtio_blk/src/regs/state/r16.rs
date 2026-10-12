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
use super::super::io::RegIo;
use super::types::Regs;
use crate::regs::pio;

impl Regs {
    #[inline]
    pub unsafe fn r16(self, offset: usize) -> u16 {
        match self.io {
            RegIo::Mmio(base) => nonos_libc::mmio_read16(base.add(offset) as usize),
            RegIo::Pio(grant) => pio::read(grant, offset, 2) as u16,
        }
    }
}
