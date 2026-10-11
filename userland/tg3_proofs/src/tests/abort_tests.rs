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

//! tg3_abort_hw on a window where every block is running: each enable bit
//! cleared, the MAC's transmit DMA and the transmitter off, the work queues
//! reset and the status block cleared, with interrupts masked first.

use crate::constants::blocks::*;
use crate::constants::grc::FTQ_RESET;
use crate::constants::hostcc::HOSTCC_MODE;
use crate::constants::layout::STATUS_AT;
use crate::constants::mac::*;
use crate::constants::mailbox::int_mbox;
use crate::init::abort;

use super::build::{LOM, REV_57766};
use super::memory::Memory;
use super::model::window;

const BLOCKS: [usize; 14] = [
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
    HOSTCC_MODE,
    WDMAC_MODE,
    MAC_TX_MODE,
];

#[test]
fn every_engine_is_stopped_and_the_queues_reset() {
    let bar = window();
    let mut m = Memory::new();
    let mut d = m.driver(&bar, REV_57766, LOM);
    // SAFETY: every offset is inside the window.
    unsafe {
        for at in BLOCKS {
            d.regs.w32(at, 0x0000_0006);
        }
        d.regs.w32(FTQ_RESET, 0x1234);
    }
    d.modes.rx_mode = RX_MODE_ENABLE | RX_MODE_KEEP_VLAN_TAG;
    d.modes.mac_mode = MAC_MODE_TDE_ENABLE | MAC_MODE_RDE_ENABLE;
    m.put32(STATUS_AT + 16, 0xFFFF_FFFF);
    abort::run(&mut d);
    for at in BLOCKS {
        assert_eq!(bar.wrote32(at), MODE_ATTN_ENABLE, "{at:#x}");
    }
    assert_eq!(bar.wrote32(MAC_RX_MODE), RX_MODE_KEEP_VLAN_TAG);
    assert_eq!(bar.wrote32(MAC_MODE), MAC_MODE_RDE_ENABLE);
    assert_eq!(bar.wrote32(FTQ_RESET), 0);
    assert_eq!(bar.wrote32(int_mbox(0)), 1);
    assert_eq!(m.get32(STATUS_AT + 16), 0);
}
