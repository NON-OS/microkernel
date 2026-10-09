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

use super::lpss_init::lpss_init;
use super::program_clock::program_clock;
use super::{fifo_depths, BusSetup, InitState};
use crate::constants::*;
use crate::regs::Regs;
use crate::transaction::control;

pub fn bring_up(regs: Regs, setup: BusSetup) -> Result<InitState, &'static str> {
    if let Some(base) = setup.lpss_base {
        lpss_init(regs, base);
    }
    // Prove MMIO actually reaches the DesignWare core before trusting any other
    // register. All-zeros means the function is unpowered or clock-gated (every
    // later read would return 0 too, so disable() below would "succeed"
    // instantly and a dead controller would bring up clean); all-ones means the
    // BAR is not decoding. Either way this is a power/mapping fault, not a
    // device fault, and binding the controller would only mask it.
    let comp_type = regs.read32(IC_COMP_TYPE);
    if comp_type == 0 || comp_type == 0xFFFF_FFFF {
        return Err("i2c-pci: mmio dead, comp_type read 0/all-ones");
    }
    // Linux i2c_dw_check_component_type: anything else is not a DesignWare
    // I2C core (or is one behind a byte-swapping or 16-bit bus this driver
    // does not implement).
    if comp_type != IC_COMP_TYPE_VALUE {
        return Err("i2c-pci: not a DesignWare I2C core (IC_COMP_TYPE mismatch)");
    }
    if setup.lpss_base.is_some() {
        let caps = regs.read32(LPSS_PRIV_CAPS);
        if (caps >> LPSS_PRIV_CAPS_TYPE_SHIFT) & LPSS_PRIV_CAPS_TYPE_MASK != LPSS_DEV_I2C {
            return Err("i2c-pci: LPSS function is a UART or SPI, not I2C");
        }
    }
    control::disable(regs).map_err(|_| "i2c-pci: controller disable timeout")?;
    let speed = if setup.standard_mode { IC_CON_SPEED_STD } else { IC_CON_SPEED_FAST };
    // 7-bit addressing as master (IC_CON bit 4 and IC_TAR bit 12 clear), with
    // RESTART enabled so a register read is write, repeated START, read.
    regs.write32(IC_CON, IC_CON_MASTER_MODE | speed | IC_CON_RESTART_EN | IC_CON_SLAVE_DISABLE);
    // The DesignWare master emits no SCL clock until the HCNT/LCNT pairs are
    // programmed. Both speed modes are set while the controller is disabled;
    // IC_CON selects one, but both pairs are required to be valid.
    program_clock(regs, setup.clock_hz);
    regs.write32(IC_RX_TL, 0);
    regs.write32(IC_TX_TL, 0);
    regs.write32(IC_INTR_MASK, 0);
    let _ = regs.read32(IC_CLR_INTR);
    let comp_param = regs.read32(IC_COMP_PARAM_1);
    let (tx_depth, rx_depth) = fifo_depths(comp_param);
    let enabled = regs.read32(IC_ENABLE_STATUS);
    let status = regs.read32(IC_STATUS);
    Ok(InitState { comp_type, comp_param, tx_depth, rx_depth, enabled, status })
}
