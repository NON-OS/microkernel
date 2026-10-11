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

//! tg3_abort_hw: the receiver off, every block stopped in Linux's order, the
//! MAC's transmit DMA and the transmitter off, then the work queues reset.
//! Linux runs it silently before a reset, so a block that does not stop in
//! time is not an error; the chip reset that follows stops it.

use crate::constants::blocks::*;
use crate::constants::grc::FTQ_RESET;
use crate::constants::hostcc::HOSTCC_MODE;
use crate::constants::mac::*;
use crate::setup::Driver;

use super::blocks::{settle, stop};
use super::power;

/// The blocks a 5705_PLUS part can stop, before the transmitter. RCVLSC,
/// DMAC, MBFREE, BUFMGR and MEMARB cannot be on such a part; tg3_stop_block
/// returns at once for them.
const BEFORE_TX: [usize; 11] = [
    RCVBDI_MODE,
    RCVLPC_MODE,
    RCVDBDI_MODE,
    RCVDCC_MODE,
    RCVCC_MODE,
    SNDBDS_MODE,
    SNDBDI_MODE,
    SNDDATAI_MODE,
    RDMAC_MODE,
    SNDDATAC_MODE,
    SNDBDC_MODE,
];

pub fn run(d: &mut Driver) {
    let regs = d.regs;
    power::disable_ints(&regs);
    d.modes.rx_mode &= !RX_MODE_ENABLE;
    d.modes.mac_mode &= !MAC_MODE_TDE_ENABLE;
    d.modes.tx_mode &= !TX_MODE_ENABLE;
    let m = d.modes;
    // SAFETY, here and for MAC_MODE below: `regs` carries the broker MmioMap
    // base for BAR0, and both are 32-bit MAC registers inside it.
    unsafe { settle(|| regs.w32f(MAC_RX_MODE, m.rx_mode)) };
    for at in BEFORE_TX {
        stop(&regs, at, MODE_ENABLE);
    }
    unsafe { settle(|| regs.w32f(MAC_MODE, m.mac_mode)) };
    stop(&regs, MAC_TX_MODE, TX_MODE_ENABLE);
    stop(&regs, HOSTCC_MODE, MODE_ENABLE);
    stop(&regs, WDMAC_MODE, MODE_ENABLE);
    // SAFETY: `regs` carries the broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it.
    unsafe {
        regs.w32(FTQ_RESET, 0xFFFF_FFFF);
        regs.w32(FTQ_RESET, 0);
    }
    d.status.clear();
}
