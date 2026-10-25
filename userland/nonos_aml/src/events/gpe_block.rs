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

//! A GPE register block (ACPI 6.5, 4.8.4.1): its first half is the status
//! registers and its second the enable registers, one bit per event, read
//! and written a byte at a time. GPE0's events number from 0, GPE1's from
//! the FADT's GPE1_BASE.

use super::gas::Block;
use super::io::EventIo;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpeBlock {
    pub port: u16,
    /// Bytes in each half.
    pub half: u8,
    /// The number of the event at bit 0 of the first byte.
    pub base: u16,
}

impl GpeBlock {
    pub fn new(b: Block, base: u16) -> Self {
        GpeBlock { port: b.port, half: b.len / 2, base }
    }

    /// The events this block holds, as [base, end).
    pub fn range(&self) -> (u16, u16) {
        (self.base, self.base.saturating_add(u16::from(self.half) * 8))
    }

    pub fn holds(&self, gpe: u16) -> bool {
        let (lo, hi) = self.range();
        lo <= gpe && gpe < hi
    }

    /// The status port, enable port and bit of `gpe`, if this block has it.
    pub fn locate(&self, gpe: u16) -> Option<(u16, u16, u8)> {
        if !self.holds(gpe) {
            return None;
        }
        let i = gpe - self.base;
        let status = self.port + i / 8;
        Some((status, status + u16::from(self.half), 1 << (i % 8)))
    }

    /// Set or clear one event's enable bit, leaving the others as they are.
    pub fn set_enabled(&self, io: &mut impl EventIo, gpe: u16, on: bool) -> bool {
        let Some((_, en, bit)) = self.locate(gpe) else { return false };
        let Some(v) = io.read(en, 1) else { return false };
        let v = if on { v | u16::from(bit) } else { v & !u16::from(bit) };
        io.write(en, 1, v)
    }

    /// Status is write-one-to-clear.
    pub fn clear(&self, io: &mut impl EventIo, gpe: u16) -> bool {
        self.locate(gpe).is_some_and(|(sts, _, bit)| io.write(sts, 1, u16::from(bit)))
    }
}
