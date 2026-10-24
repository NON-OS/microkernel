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

//! The schedule memory: one region the controller reads by bus address,
//! the CPU by offset. Every word is read and written whole and volatile,
//! since the controller writes TD status while the CPU looks.

pub trait Mem {
    /// The bus address of offset `off`, below 4 GiB.
    fn bus(&self, off: usize) -> u32;
    fn rd(&self, off: usize) -> u32;
    fn wr(&self, off: usize, v: u32);
    fn read_bytes(&self, off: usize, out: &mut [u8]);
    fn write_bytes(&self, off: usize, src: &[u8]);
}
