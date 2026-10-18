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

//! Stopping the engines after a failure, at teardown and at claim, and
//! clearing the SD engine's error state, as rtsx_pci_stop_cmd and
//! sd_clear_error do. After a failure or at teardown their own failures add
//! nothing. At claim the stop must be seen (setup/master.rs): the DMA reset
//! and ring flush go through HAIMR, which answers with the data the chip
//! took only once it has taken them, after the two stop writes.

use crate::error::Result;
use crate::hw::write_register;
use crate::regs::card::{CARD_STOP, SD_CLR_ERR, SD_STOP};
use crate::regs::host::{HCBCTLR, HDBCTLR, STOP_CMD, STOP_DMA};
use crate::regs::pm::{DMACTL, DMA_RST, RBCTL, RB_FLUSH};
use crate::setup::Driver;

pub fn stop(drv: &Driver) {
    let _ = stopped(drv);
}

/// Both engines stopped, and Ok only once the chip has answered the reset
/// and the flush that follow the stops.
pub fn stopped(drv: &Driver) -> Result<()> {
    drv.regs.w32(HCBCTLR, STOP_CMD);
    drv.regs.w32(HDBCTLR, STOP_DMA);
    let reset = write_register(drv.regs, DMACTL, DMA_RST, DMA_RST);
    let flush = write_register(drv.regs, RBCTL, RB_FLUSH, RB_FLUSH);
    reset.and(flush)
}

pub fn clear_error(drv: &Driver) {
    let _ = write_register(drv.regs, CARD_STOP, SD_STOP | SD_CLR_ERR, SD_STOP | SD_CLR_ERR);
}
