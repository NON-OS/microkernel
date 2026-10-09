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

//! A card that just arrived, from power to ready, in the order the MMC core
//! drives this host (mmc_power_up, then mmc_attach_sd): 1-bit bus, slot
//! power, default timing, the identification clock, 3.3 V signalling, the
//! initial clocks stopped, then identification. Each step that stops is
//! named in the log.

use super::attach::attach;
use super::clock::switch_clock;
use super::identify::identify;
use super::info::Card;
use super::power::power_on;
use super::timing::default_timing;
use super::voltage::signal_3v3;
use crate::clock::sleep_ms;
use crate::error::Result;
use crate::hw::write_register;
use crate::log::step;
use crate::regs::clk::SSC_DEPTH_500K;
use crate::regs::sd::*;
use crate::setup::Driver;

/// The MMC core's power_delay_ms default, after power-up and power-on.
const POWER_DELAY_MS: u64 = 10;

pub fn bring_up(drv: &mut Driver) -> Result<Card> {
    let regs = drv.regs;
    step(b"1-bit bus", write_register(regs, SD_CFG1, SD_BUS_WIDTH_MASK, SD_BUS_WIDTH_1BIT))?;
    step(b"slot power", power_on(drv))?;
    step(b"default timing", default_timing(drv))?;
    step(b"identification clock", switch_clock(drv, 400_000, SSC_DEPTH_500K, true))?;
    step(b"3.3 V signalling", signal_3v3(drv))?;
    sleep_ms(POWER_DELAY_MS);
    step(b"initial clocks", write_register(regs, SD_BUS_STAT, SD_CLK_TOGGLE_EN, 0))?;
    sleep_ms(POWER_DELAY_MS);
    let ocr = step(b"identification", identify(drv))?;
    step(b"card setup", attach(drv, ocr))
}
