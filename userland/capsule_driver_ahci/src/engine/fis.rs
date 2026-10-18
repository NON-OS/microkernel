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

use crate::constants::ata::{FIS_H2D_COMMAND, FIS_TYPE_REG_H2D};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FisH2D {
    pub fis_type: u8,
    pub pm: u8,
    pub command: u8,
    pub featurel: u8,
    pub lba0: u8,
    pub lba1: u8,
    pub lba2: u8,
    pub device: u8,
    pub lba3: u8,
    pub lba4: u8,
    pub lba5: u8,
    pub featureh: u8,
    pub countl: u8,
    pub counth: u8,
    pub icc: u8,
    pub control: u8,
    pub rsv: [u8; 4],
}

const _: () = assert!(core::mem::size_of::<FisH2D>() == 20);

impl FisH2D {
    /// The Register H2D FIS carrying ATA command `cmd` for `sectors`
    /// sectors at `lba`, with `device` in the device register.
    pub fn command(cmd: u8, lba: u64, sectors: u32, device: u8) -> Self {
        Self {
            fis_type: FIS_TYPE_REG_H2D,
            pm: FIS_H2D_COMMAND,
            command: cmd,
            featurel: 0,
            lba0: lba as u8,
            lba1: (lba >> 8) as u8,
            lba2: (lba >> 16) as u8,
            device,
            lba3: (lba >> 24) as u8,
            lba4: (lba >> 32) as u8,
            lba5: (lba >> 40) as u8,
            featureh: 0,
            countl: sectors as u8,
            counth: (sectors >> 8) as u8,
            icc: 0,
            control: 0,
            rsv: [0; 4],
        }
    }
}
