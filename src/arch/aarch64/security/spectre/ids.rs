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

//! The ID registers the decision reads, taken once per CPU and passed in, so
//! the decision reads no register and is proved on a host. Field positions
//! are the Arm ARM's register descriptions, listed for the owner to check.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Ids {
    pub(crate) pfr0: u64,
    pub(crate) pfr1: u64,
    pub(crate) mmfr1: u64,
    pub(crate) mmfr2: u64,
    pub(crate) isar2: u64,
    pub(crate) midr: u64,
}

const fn field(register: u64, shift: u32) -> u8 {
    ((register >> shift) & 0xF) as u8
}

impl Ids {
    /// ID_AA64PFR0_EL1.CSV2, 59:56. 1 is FEAT_CSV2, 3 is FEAT_CSV2_3.
    pub(crate) const fn csv2(&self) -> u8 {
        field(self.pfr0, 56)
    }

    /// ID_AA64PFR0_EL1.CSV3, 63:60.
    pub(crate) const fn csv3(&self) -> u8 {
        field(self.pfr0, 60)
    }

    /// ID_AA64PFR1_EL1.SSBS, 7:4. 1 is PSTATE.SSBS with the immediate MSR.
    pub(crate) const fn ssbs(&self) -> u8 {
        field(self.pfr1, 4)
    }

    /// ID_AA64MMFR1_EL1.ECBHB, 63:60.
    pub(crate) const fn ecbhb(&self) -> u8 {
        field(self.mmfr1, 60)
    }

    /// ID_AA64MMFR2_EL1.E0PD, 63:60.
    pub(crate) const fn e0pd(&self) -> u8 {
        field(self.mmfr2, 60)
    }

    /// ID_AA64ISAR2_EL1.CLRBHB, 31:28.
    pub(crate) const fn clrbhb(&self) -> u8 {
        field(self.isar2, 28)
    }

    /// MIDR_EL1.Implementer, 31:24.
    pub(crate) const fn implementer(&self) -> u8 {
        (self.midr >> 24) as u8
    }

    /// MIDR_EL1.PartNum, 15:4.
    pub(crate) const fn part(&self) -> u16 {
        ((self.midr >> 4) & 0xFFF) as u16
    }
}
