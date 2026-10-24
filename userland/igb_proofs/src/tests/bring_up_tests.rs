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
//! The whole bring-up against a modelled 82576, QEMU's part: reset completes
//! and the NVM loads, the PHY answers MDIC at address 1, and the enables
//! read back as written.

use std::sync::atomic::Ordering;

use nonos_libc::{entropy, said};

use crate::bring::finish;
use crate::facts::mac::*;
use crate::facts::regs::*;
use crate::link::LinkState;

use super::memory::Memory;
use super::model::{live, window};
use super::part_model::part;
use super::phy_model::{bmcr, BMCR_POWERED_DOWN};

#[test]
fn the_part_comes_up_under_the_drawn_address_and_says_so() {
    let _on = entropy(true);
    let bar = window();
    bar.present32(ral(1), 0xDEAD_BEEF);
    bar.present32(ral(23) + 4, RAH_AV | 0x1234);
    bar.present32(REG_MTA_BASE + 20, 0xFFFF_FFFF);
    let held = bmcr(BMCR_POWERED_DOWN);
    let _part = live(&bar, part(held.clone(), None));
    let mut mem = Memory::new();
    let Ok(d) = finish(mem.driver(&bar)) else { panic!("a working part comes up") };
    let mac = d.mac;
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    assert_eq!(bar.wrote32(ral(0)), u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]));
    assert_eq!(bar.wrote32(ral(0) + 4), u32::from_le_bytes([mac[4], mac[5], 0, 0]) | RAH_AV);
    assert_eq!(bar.wrote32(ral(1)), 0, "old RAR1 entry gone");
    assert_eq!(bar.wrote32(0x54E0 + 7 * 8 + 4), 0, "RAR23, the 82576's last, gone");
    assert_eq!(bar.wrote32(REG_MTA_BASE + 20), 0, "MTA cleared");
    assert_eq!(held.load(Ordering::SeqCst), 0x1340, "PHY powered up");
    assert_ne!(bar.wrote32(REG_CTRL) & CTRL_SLU, 0);
    assert_eq!(bar.wrote32(REG_PHPM), 0, "no PHPM on an 82576");
    assert_eq!(bar.wrote32(REG_RCTL) & 0x2, 0x2, "receiver on");
    assert_eq!(bar.wrote32(REG_TCTL) & 0x2, 0x2, "transmitter on");
    assert_eq!(bar.wrote32(REG_RDT), 31);
    assert_eq!(d.link, Some(LinkState { up: false, mbps: 10, full: false }));
    let text: Vec<String> = mac.iter().map(|b| format!("{b:02x}")).collect();
    let want = format!("igb: up 10c9 {} link=down\n", text.join(":"));
    assert_eq!(said().last(), Some(&want));
}
