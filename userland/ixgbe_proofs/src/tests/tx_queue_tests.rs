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
//! ixgbe_configure_tx for queue 0 at 0x6000, IXGBE_TDBAL(0): the DMA
//! transmit engine on before the queue, then the queue.
//!
//! Not proven here: a queue whose enable never reads back. The model shares
//! memory with the driver, so the driver's own write reads back before any
//! concurrent model can undo it; that refusal rests on `until`, whose
//! timeout is proven in wait_tests.

use crate::constants::ctrl::DMATXCTL_TE;
use crate::constants::regs::*;
use crate::init::tx_queue::program;
use crate::queue::layout::TxDesc;

use super::memory::Memory;
use super::model::{regs, window};

const RING: u64 = 0x0000_0001_2345_6000;

#[test]
fn queue_zero_is_programmed_at_0x6000_after_the_engine_and_enabled() {
    let bar = window();
    let mut mem = Memory::new();
    mem.tx[3] = TxDesc { buffer_addr: 1, cmd_type_len: 2, olinfo_status: 3 };
    bar.present32(REG_DMATXCTL, 0x0000_0008);
    assert_eq!(program(&regs(&bar), &mem.tx_ring(), RING), Ok(()));
    assert_eq!(bar.wrote32(REG_DMATXCTL), 0x0000_0008 | DMATXCTL_TE, "TE, the rest kept");
    assert_eq!(bar.wrote32(0x6000), 0x2345_6000, "TDBAL");
    assert_eq!(bar.wrote32(0x6004), 0x0000_0001, "TDBAH");
    assert_eq!(bar.wrote32(0x6008), 512, "TDLEN, 32 x 16 bytes");
    assert_eq!(bar.wrote32(REG_TDH), 0);
    assert_eq!(bar.wrote32(REG_TDT), 0);
    let txdctl = bar.wrote32(0x6028);
    assert_eq!(txdctl, 32 | (1 << 8) | (1 << 16) | (1 << 25), "PTHRESH HTHRESH WTHRESH ENABLE");
    assert_eq!(mem.tx[3].cmd_type_len, 0, "ring cleared before the part reads it");
}
