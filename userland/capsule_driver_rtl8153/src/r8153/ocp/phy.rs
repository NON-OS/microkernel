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

//! The PHY through the PLA window: its OCP registers (Linux ocp_reg_read
//! and ocp_reg_write), its MII registers (r8152_mdio_read and
//! r8152_mdio_write) and its SRAM (sram_write).

use nonos_usbnet::Bus;

use super::dev::Dev;
use super::read::read_word;
use super::request::MCU_TYPE_PLA;
use super::write::write_word;
use crate::r8153::regs::phy::{OCP_BASE_MII, OCP_SRAM_ADDR, OCP_SRAM_DATA};
use crate::r8153::regs::pla::OCP_GPHY_BASE;

/// The PLA addresses where the selected 4 KiB PHY page shows.
const WINDOW: u16 = 0xb000;

/// The window address of PHY register `addr`, moving the window to its
/// page first when it shows another one.
fn window<B: Bus>(dev: &mut Dev<B>, addr: u16) -> Result<u16, i32> {
    let base = addr & 0xf000;
    if dev.ocp_base != Some(base) {
        write_word(dev, MCU_TYPE_PLA, OCP_GPHY_BASE, base)?;
        dev.ocp_base = Some(base);
    }
    Ok((addr & 0x0fff) | WINDOW)
}

pub fn phy_read<B: Bus>(dev: &mut Dev<B>, addr: u16) -> Result<u16, i32> {
    let at = window(dev, addr)?;
    read_word(dev, MCU_TYPE_PLA, at)
}

pub fn phy_write<B: Bus>(dev: &mut Dev<B>, addr: u16, v: u16) -> Result<(), i32> {
    let at = window(dev, addr)?;
    write_word(dev, MCU_TYPE_PLA, at, v)
}

/// MII register `reg` sits at OCP_BASE_MII plus two bytes a register.
pub fn mdio_read<B: Bus>(dev: &mut Dev<B>, reg: u16) -> Result<u16, i32> {
    phy_read(dev, OCP_BASE_MII + reg * 2)
}

pub fn mdio_write<B: Bus>(dev: &mut Dev<B>, reg: u16, v: u16) -> Result<(), i32> {
    phy_write(dev, OCP_BASE_MII + reg * 2, v)
}

pub fn sram_write<B: Bus>(dev: &mut Dev<B>, addr: u16, v: u16) -> Result<(), i32> {
    phy_write(dev, OCP_SRAM_ADDR, addr)?;
    phy_write(dev, OCP_SRAM_DATA, v)
}
