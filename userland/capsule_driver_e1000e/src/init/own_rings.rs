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

//! Before the I219 ring flush runs either unit: both stopped and read back
//! off, both rings named again at slot 0 of this driver's own frames, and
//! only then Bus Master on. The flush runs only once the part has run on
//! these rings (`flush_rings`); still, no unit may run, nor a ring name any
//! memory but this driver's, once the part can reach memory.

use crate::constants::regs::{REG_RCTL, REG_TCTL};
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::setup::{command, Driver};

use super::{quiesce::stop_units, rx_setup, tx_setup};

pub fn run(d: &Driver) -> Result<(), &'static str> {
    stop_units(&d.regs, d.family);
    // SAFETY: `d.regs` is the broker-mapped BAR0 window; RCTL and TCTL are
    // 4-byte registers inside it on every family.
    let on = unsafe { (d.regs.r32(REG_RCTL) & RCTL_EN) | (d.regs.r32(REG_TCTL) & TCTL_EN) };
    if on != 0 {
        return Err("RCTL/TCTL still on after the stop, ring flush skipped");
    }
    rx_setup::program(&d.regs, &d.rx, d.rx_ring_device_addr);
    tx_setup::program(&d.regs, &d.tx, d.tx_ring_device_addr);
    command(d.device_id, d.claim_epoch, true)
}
