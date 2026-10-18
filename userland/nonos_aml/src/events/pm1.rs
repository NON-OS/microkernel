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

//! The PM1 event registers (ACPI 6.5, 4.8.3.1): a status register and an
//! enable register, each half of the block, in an A block and an optional B
//! block. Reads are the two ORed together and writes go to both (4.8.1.1).

use super::gas::Block;
use super::io::EventIo;
pub use super::pm1_bits::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pm1 {
    pub a: Block,
    pub b: Option<Block>,
}

/// The register's width: half the block, which is at least four bytes, and
/// never wider than the 16 bits the layout defines.
fn half(b: &Block) -> (u8, u8) {
    let h = b.len / 2;
    (h, h.min(2))
}

impl Pm1 {
    fn blocks(&self) -> impl Iterator<Item = &Block> {
        core::iter::once(&self.a).chain(self.b.iter())
    }

    fn read(&self, io: &mut impl EventIo, enable: bool) -> Option<u16> {
        self.blocks().try_fold(0u16, |acc, b| {
            let (h, w) = half(b);
            Some(acc | io.read(b.port + if enable { u16::from(h) } else { 0 }, w)?)
        })
    }

    fn write(&self, io: &mut impl EventIo, enable: bool, v: u16) -> bool {
        self.blocks().all(|b| {
            let (h, w) = half(b);
            io.write(b.port + if enable { u16::from(h) } else { 0 }, w, v)
        })
    }

    pub fn status(&self, io: &mut impl EventIo) -> Option<u16> {
        self.read(io, false)
    }

    /// Status bits are write-one-to-clear: exactly `bits` are cleared.
    pub fn clear(&self, io: &mut impl EventIo, bits: u16) -> bool {
        self.write(io, false, bits)
    }

    pub fn set_enable(&self, io: &mut impl EventIo, bits: u16) -> bool {
        self.write(io, true, bits)
    }
}
