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

//! The model behind the engine's register trait: registers through the
//! latch rules, GGTT entries straight into the table.

use super::gen9::Gen9;
use crate::engine::access::Mmio;
use crate::engine::ggtt::GGTT_OFFSET;

impl Mmio for Gen9 {
    fn read(&mut self, off: u32) -> u32 {
        self.on_read(off)
    }
    fn write(&mut self, off: u32, v: u32) {
        self.on_write(off, v)
    }
    fn write64(&mut self, off: u32, v: u64) {
        let i = ((off - GGTT_OFFSET) / 8) as usize;
        self.ggtt[i] = v;
    }
    fn read64(&mut self, off: u32) -> u64 {
        self.ggtt[((off - GGTT_OFFSET) / 8) as usize]
    }
}
