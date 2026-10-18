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

//! A reset part put on the air: the receive filter under the station
//! address, both rings named, Bus Master on, and only then the engines. Bus
//! Master goes on after the rings so a part never masters while its ring
//! registers still name a previous owner's frames; the engines go on after
//! it so neither starts a fetch the bus would refuse.

use crate::constants::regs::{REG_RCTL, REG_TCTL};
use crate::constants::status::{RCTL_EN, TCTL_EN};
use crate::setup::{command, Driver};

use super::{mac_filter, rx_setup, tx_setup};

pub fn run(d: &mut Driver) -> Result<(), &'static str> {
    mac_filter::program(&d.regs, &d.mac);
    rx_setup::program(&d.regs, &d.rx, d.rx_ring_device_addr);
    tx_setup::program(&d.regs, &d.tx, d.tx_ring_device_addr);
    command(d.device_id, d.claim_epoch, true)?;
    // SAFETY: `d.regs` carries the broker MmioMap base for BAR0; RCTL and
    // TCTL are 32-bit aligned offsets in it.
    unsafe {
        d.regs.w32(REG_RCTL, d.regs.r32(REG_RCTL) | RCTL_EN);
        d.regs.w32(REG_TCTL, d.regs.r32(REG_TCTL) | TCTL_EN);
    }
    Ok(())
}
