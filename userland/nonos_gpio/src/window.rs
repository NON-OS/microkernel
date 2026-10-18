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

//! A controller's register window: dword reads and writes at an offset.
//! The capsule backs it with a mapped MMIO window; the proofs with a model
//! that behaves as the registers do (write-one-to-clear status bits among
//! them).

pub trait Window {
    fn read(&mut self, offset: u32) -> u32;
    fn write(&mut self, offset: u32, value: u32);
}

/// Read, change the bits `mask` selects to `bits`, write back.
pub fn update(w: &mut impl Window, offset: u32, mask: u32, bits: u32) {
    let v = w.read(offset);
    w.write(offset, (v & !mask) | (bits & mask));
}
