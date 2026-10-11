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

//! A whole bring-up on a window in host memory with the modelled PHY: the
//! rings and status block given to the part, every mailbox masked, the
//! drawn address in all four slots, the engines and receiver on, and every
//! configuration write one the kernel broker allows. No bootcode answers
//! here, so both firmware waits run to their end, as on a board without it.

use std::sync::{Arc, Mutex};

use crate::constants::hostcc::*;
use crate::constants::layout::STATUS_AT;
use crate::constants::mac::*;
use crate::constants::mailbox::*;
use crate::constants::ring::*;
use crate::init::{engines::MAC_MODE_ENABLES, mac_setup, run};
use crate::link::LinkState;

use super::build::{LOM, REV_57766};
use super::config_replay::{replay_against_the_broker, seed, seeded};
use super::memory::{Memory, RINGS_PHYS};
use super::model::{live, window};
use super::phy_model::phy;
use super::phy_states::linked;

#[test]
fn a_bring_up_leaves_the_part_running() {
    let bar = window();
    let mut m = Memory::new();
    let mut d = m.driver(&bar, REV_57766, LOM);
    seed();
    let start = seeded();
    let _part = live(&bar, phy(Arc::new(Mutex::new(linked(0x0700)))));
    let link = run::bring_up(&mut d).expect("bring-up");
    assert_eq!(link, LinkState { up: true, mbps: 1000, full: true });
    let r = |at| bar.wrote32(at);
    assert_eq!(r(RCVDBDI_STD_BD + 4), RINGS_PHYS as u32);
    assert_eq!(r(RCVDBDI_STD_BD + 8), 0x0200_1800);
    assert_eq!(r(RX_STD_PROD_IDX_REG), 64);
    assert_eq!(r(HOSTCC_STATUS_BLK_HOST_ADDR + 4), (RINGS_PHYS + STATUS_AT as u64) as u32);
    assert_eq!(r(HOSTCC_MODE), HOSTCC_MODE_ENABLE | HOSTCC_MODE_32BYTE);
    assert!((0..5).all(|n| r(int_mbox(n)) == 1), "interrupts stay masked");
    let (high, low) = mac_setup::address_words(&d.mac);
    for n in 0..4 {
        assert_eq!((r(MAC_ADDR_0_HIGH + 8 * n), r(MAC_ADDR_0_LOW + 8 * n)), (high, low));
    }
    assert_eq!(r(MAC_MODE), MAC_MODE_ENABLES | MAC_MODE_PORT_MODE_GMII);
    let rx = RX_MODE_ENABLE | RX_MODE_IPV6_CSUM_ENABLE | RX_MODE_KEEP_VLAN_TAG;
    assert_eq!(r(MAC_RX_MODE), rx | RX_MODE_FLOW_CTRL_ENABLE);
    let tx = TX_MODE_ENABLE | TX_MODE_MBUF_LOCKUP_FIX | TX_MODE_FLOW_CTRL_ENABLE;
    assert_eq!(r(MAC_TX_MODE), tx);
    assert_eq!(r(MAC_RX_MTU_SIZE), 1522);
    assert!(replay_against_the_broker(&start) >= 5);
}
