// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The ID register fields the loader's security checks read, from the Arm
//! Architecture Reference Manual. Pure, so it is held on the host
//! (loader_arm_proofs). Each field is four bits; zero means not implemented.

/// ID_AA64ISAR0_EL1, ID_AA64MMFR0_EL1, ID_AA64MMFR1_EL1, ID_AA64MMFR2_EL1.
#[derive(Clone, Copy, Debug, Default)]
pub struct IdRegs {
    pub isar0: u64,
    pub mmfr0: u64,
    pub mmfr1: u64,
    pub mmfr2: u64,
}

fn field(reg: u64, at: u32) -> u64 {
    (reg >> at) & 0xF
}

impl IdRegs {
    /// FEAT_PAN: ID_AA64MMFR1_EL1.PAN, bits 23:20.
    pub fn pan(&self) -> bool {
        field(self.mmfr1, 20) != 0
    }
    /// FEAT_RNG: ID_AA64ISAR0_EL1.RNDR, bits 63:60.
    pub fn rndr(&self) -> bool {
        field(self.isar0, 60) != 0
    }
    /// FEAT_AES: ID_AA64ISAR0_EL1.AES, bits 7:4.
    pub fn aes(&self) -> bool {
        field(self.isar0, 4) != 0
    }
    /// FEAT_SHA256: ID_AA64ISAR0_EL1.SHA2, bits 15:12.
    pub fn sha2(&self) -> bool {
        field(self.isar0, 12) != 0
    }
    /// The physical address size, from ID_AA64MMFR0_EL1.PARange (bits 3:0);
    /// 0 for an encoding the manual does not define, which no check passes.
    pub fn physical_bits(&self) -> u8 {
        match field(self.mmfr0, 0) {
            0 => 32,
            1 => 36,
            2 => 40,
            3 => 42,
            4 => 44,
            5 => 48,
            6 => 52,
            7 => 56,
            _ => 0,
        }
    }
    /// The virtual address size: 52 with FEAT_LVA (ID_AA64MMFR2_EL1.VARange,
    /// bits 19:16, is 1), else 48.
    pub fn virtual_bits(&self) -> u8 {
        if field(self.mmfr2, 16) == 1 {
            52
        } else {
            48
        }
    }
}
