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

//! A PCIe PHY register write (__rtsx_pci_write_phy_register): data, address,
//! start, then wait for the busy bit to clear.

use super::mmio::Regs;
use super::reg::{read_register, write_register};
use crate::clock::{now, Budget};
use crate::error::{Result, RtsxError};
use crate::regs::phy::{PHYADDR, PHYDATA0, PHYDATA1, PHYRWCTL, PHY_BUSY, PHY_WRITE_START};

/// Linux polls up to 100000 times; on the clock that is generous at 50 ms.
const PHY_MS: u64 = 50;

pub fn write_phy(regs: Regs, addr: u8, value: u16) -> Result<()> {
    write_register(regs, PHYDATA0, 0xFF, value as u8)?;
    write_register(regs, PHYDATA1, 0xFF, (value >> 8) as u8)?;
    write_register(regs, PHYADDR, 0xFF, addr)?;
    write_register(regs, PHYRWCTL, 0xFF, PHY_WRITE_START)?;
    let budget = Budget::begin(now(), PHY_MS);
    loop {
        if read_register(regs, PHYRWCTL)? & PHY_BUSY == 0 {
            return Ok(());
        }
        if budget.spent(now()) {
            return Err(RtsxError::PhyTimeout);
        }
    }
}
