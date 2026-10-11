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
//! A whole bring-up against a part that finishes its resets and loads its
//! EEPROM: the drawn address in the filter, the MAC settings, both queues
//! and the receiver on, the 82599's security block let go after, and the
//! line the owner reads.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_libc::{entropy, said};

use crate::constants::ctrl::*;
use crate::constants::regs::*;
use crate::init::finish;
use crate::link::LinkState;

use super::memory::Memory;
use super::model::{live, window};
use super::part_model::part;

#[test]
fn the_part_comes_up_under_the_drawn_address_and_says_so() {
    let _on = entropy(true);
    let bar = window();
    bar.present32(ral(1), 0xDEAD_BEEF);
    bar.present32(ral(127) + 4, RAH_AV | 0x1234);
    bar.present32(REG_MTA_BASE + 20, 0xFFFF_FFFF);
    bar.present32(REG_AUTOC, 0x0000_C000);
    bar.present32(REG_LINKS, 0x7000_0000);
    let resets = Arc::new(AtomicU32::new(0));
    let _part = live(&bar, part(0x0000_A000, resets.clone()));
    let mut mem = Memory::new();
    let Ok(d) = finish(mem.driver(&bar)) else { panic!("a working part comes up") };
    let mac = d.mac;
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    assert_eq!(bar.wrote32(ral(0)), u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]));
    assert_eq!(bar.wrote32(ral(0) + 4), u32::from_le_bytes([mac[4], mac[5], 0, 0]) | RAH_AV);
    assert_eq!((bar.wrote32(ral(1)), bar.wrote32(ral(127) + 4)), (0, 0), "old entries gone");
    assert_eq!(bar.wrote32(REG_MTA_BASE + 20), 0, "MTA cleared");
    assert_eq!(resets.load(Ordering::SeqCst), 2);
    assert_eq!(bar.wrote32(REG_AUTOC), 0x0000_C000 | AUTOC_AN_RESTART, "link mode put back");
    let ext = bar.wrote32(REG_CTRL_EXT);
    assert_eq!(ext & (CTRL_EXT_DRV_LOAD | CTRL_EXT_NS_DIS), CTRL_EXT_DRV_LOAD | CTRL_EXT_NS_DIS);
    assert_ne!(bar.wrote32(REG_DMATXCTL) & DMATXCTL_TE, 0, "transmit engine on");
    assert_ne!(bar.wrote32(REG_TXDCTL) & (1 << 25), 0);
    assert_ne!(bar.wrote32(REG_RXDCTL) & (1 << 25), 0);
    assert_ne!(bar.wrote32(REG_RXCTRL) & RXCTRL_RXEN, 0, "receiver on");
    assert_eq!(bar.wrote32(REG_SECRXCTRL) & SECRXCTRL_RX_DIS, 0, "security path let go");
    assert_eq!(bar.wrote32(REG_RDT), 31);
    assert_eq!(d.link, Some(LinkState { up: true, mbps: 10_000 }));
    let text: Vec<String> = mac.iter().map(|b| format!("{b:02x}")).collect();
    let want = format!("ixgbe: up 10fb {} link=up\n", text.join(":"));
    assert_eq!(said().last(), Some(&want));
}
