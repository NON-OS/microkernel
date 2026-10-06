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

//! The whole chip bring-up in rtsx_pci_init_hw's order, each step named in
//! the log when it stops. Left out, as config space past 256 bytes and Link
//! Control are not a capsule's to write: CLKREQ# enable and the 0x70F L1
//! entry setting, both power savings only.

use super::aspm::disable_aspm;
use super::bier::enable_bus_int;
use super::extra::extra_init_hw;
use super::extra_522a::extra_init_hw_522a;
use super::ic::read_ic_version;
use super::ocp::init_ocp;
use super::phy::optimize_phy;
use super::sequence::common_sequence;
use crate::chip::Family;
use crate::clock::sleep_ms;
use crate::error::Result;
use crate::hw::write_register;
use crate::log::step;
use crate::regs::clk::{FPDCTL, SSC_POWER_DOWN};
use crate::regs::host::HCBAR;
use crate::regs::pm::ASPM_FORCE_CTL;
use crate::setup::Driver;

pub fn init_hw(drv: &mut Driver) -> Result<()> {
    step(b"IC version", read_ic_version(drv))?;
    drv.regs.w32(HCBAR, drv.resv.device_addr());
    enable_bus_int(drv);
    step(b"SSC power on", write_register(drv.regs, FPDCTL, SSC_POWER_DOWN, 0))?;
    // 200 us for the SSC power to settle.
    sleep_ms(1);
    step(b"ASPM off", disable_aspm(drv))?;
    step(b"PHY tuning", optimize_phy(drv))?;
    step(b"init sequence", common_sequence(drv))?;
    step(b"over-current protection", init_ocp(drv))?;
    step(b"chip init", extra_init_hw(drv))?;
    if drv.family == Family::Rts522a {
        step(b"RTS522A init", extra_init_hw_522a(drv))?;
        step(b"ASPM force", write_register(drv.regs, ASPM_FORCE_CTL, 0x30, 0x30))?;
    }
    Ok(())
}
