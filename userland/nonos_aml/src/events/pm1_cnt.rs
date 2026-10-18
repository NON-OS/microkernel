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

//! The PM1 control registers (ACPI 6.5, 4.8.3.2.1, table 4.18): SCI_EN says
//! the platform is in ACPI mode; GBL_RLS tells firmware the Global Lock it
//! waited for is free.

use super::gas::Block;
use super::io::EventIo;

pub const SCI_EN: u16 = 1 << 0;
pub const GBL_RLS: u16 = 1 << 2;
/// Written as zero whenever the register is written for anything else, so
/// releasing the lock never puts the machine to sleep.
pub const SLP_EN: u16 = 1 << 13;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pm1Cnt {
    pub a: Block,
    pub b: Option<Block>,
}

impl Pm1Cnt {
    fn blocks(&self) -> impl Iterator<Item = &Block> {
        core::iter::once(&self.a).chain(self.b.iter())
    }

    pub fn read(&self, io: &mut impl EventIo) -> Option<u16> {
        self.blocks().try_fold(0u16, |acc, b| Some(acc | io.read(b.port, b.len.min(2))?))
    }

    pub fn sci_enabled(&self, io: &mut impl EventIo) -> Option<bool> {
        self.read(io).map(|v| v & SCI_EN != 0)
    }

    /// GBL_RLS is write-only and reads as zero; each block keeps its other
    /// bits, and SLP_EN is never written as one here.
    pub fn release_lock(&self, io: &mut impl EventIo) -> bool {
        self.blocks().all(|b| {
            let w = b.len.min(2);
            io.read(b.port, w).is_some_and(|v| io.write(b.port, w, (v | GBL_RLS) & !SLP_EN))
        })
    }
}
