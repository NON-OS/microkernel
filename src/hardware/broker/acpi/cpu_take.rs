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

//! Reading a MkAcpiCpu record front to back; every read is bounded by the
//! record, and running out is a refusal, never a fault.

use super::cpu_layout::{ABSENT, INTEGER, REGISTER};
use super::cpu_record::Reg;

/// Spaces a processor register may name (ACPI 6.5, 5.2.3.2): memory, I/O,
/// the PCC channel, and the processor's own (fixed hardware). Anything
/// else is not a processor control and is refused.
const SPACES: [u8; 4] = [0x00, 0x01, 0x0A, 0x7F];

pub struct Take<'a> {
    pub b: &'a [u8],
    pub at: usize,
}

impl Take<'_> {
    pub fn bytes<const N: usize>(&mut self) -> Option<[u8; N]> {
        let s = self.b.get(self.at..self.at.checked_add(N)?)?;
        self.at += N;
        s.try_into().ok()
    }

    pub fn u8(&mut self) -> Option<u8> {
        self.bytes::<1>().map(|b| b[0])
    }

    pub fn u32(&mut self) -> Option<u32> {
        self.bytes().map(u32::from_le_bytes)
    }

    pub fn u64(&mut self) -> Option<u64> {
        self.bytes().map(u64::from_le_bytes)
    }

    /// Bytes the layout leaves zero must be zero, so a later version's
    /// fields are never read as this one's.
    pub fn zeros<const N: usize>(&mut self) -> Option<()> {
        self.bytes::<N>().filter(|b| b.iter().all(|x| *x == 0)).map(|_| ())
    }

    pub fn reg(&mut self) -> Option<Reg> {
        let [kind, space, width, offset, access] = self.bytes::<5>()?;
        self.zeros::<3>()?;
        let value = self.u64()?;
        let ok = match kind {
            ABSENT => space == 0 && width == 0 && offset == 0 && access == 0 && value == 0,
            REGISTER => SPACES.contains(&space) && access <= 4,
            INTEGER => space == 0 && width == 0 && offset == 0 && access == 0,
            _ => false,
        };
        ok.then_some(Reg { kind, space, width, offset, access, value })
    }

    /// A register proper: _PCT's and _CST's may not be absent or integers.
    pub fn register(&mut self) -> Option<Reg> {
        self.reg().filter(|r| r.kind == REGISTER)
    }
}
