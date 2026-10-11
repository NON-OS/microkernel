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

//! The words `decode` reads, out of SRAM.

use crate::chip::sram;
use crate::constants::sram::{
    NIC_SRAM_DATA_CFG, NIC_SRAM_DATA_CFG_2, NIC_SRAM_DATA_CFG_3, NIC_SRAM_DATA_SIG,
    NIC_SRAM_DATA_VER,
};
use crate::regs::Regs;

use super::cfg::HwCfg;
use super::decode::decode;

impl HwCfg {
    pub fn read(regs: &Regs) -> Result<HwCfg, &'static str> {
        let at = [
            NIC_SRAM_DATA_SIG,
            NIC_SRAM_DATA_CFG,
            NIC_SRAM_DATA_VER,
            NIC_SRAM_DATA_CFG_2,
            NIC_SRAM_DATA_CFG_3,
        ];
        decode(at.map(|off| sram::read(regs, off)))
    }
}
