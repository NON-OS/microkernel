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

//! A register as a test reads it, and as a test sets it: both the value a
//! read returns and the copy the scanout uses, as firmware leaves them.

use super::gen9::Gen9;

impl Gen9 {
    pub fn get(&self, off: u32) -> u32 {
        *self.regs.get(&off).unwrap_or(&0)
    }

    pub fn set(&mut self, off: u32, v: u32) {
        self.regs.insert(off, v);
        self.live.insert(off, v);
    }
}
