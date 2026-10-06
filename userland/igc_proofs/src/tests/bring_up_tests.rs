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

//! The whole bring-up against a modelled part: reset completes and the NVM
//! loads, the PHY answers MDIC, and the enables read back as written.

use nonos_devmodel::FakeBar;
use nonos_libc::{entropy, said};

use crate::constants::ctrl::*;
use crate::constants::regs::*;
use crate::init::finish;
use crate::link::LinkState;

use super::memory::Memory;
use super::model::{live, stable32, window};
use super::phy_model::{bmcr, phy, BMCR_POWERED_DOWN};

/// A part that finishes a global reset on the next look: CTRL.RST and the
/// master disable drop, and EECD reports the NVM auto-read done.
pub fn part(held: std::sync::Arc<std::sync::atomic::AtomicU32>) -> impl Fn(&FakeBar) + Send {
    let mdio = phy(held);
    move |b| {
        let ctrl = stable32(b, REG_CTRL).unwrap_or(0);
        if ctrl & CTRL_RST != 0 {
            b.present32(REG_CTRL, ctrl & !(CTRL_RST | CTRL_GIO_MASTER_DISABLE));
            b.present32(REG_EECD, EECD_AUTO_RD);
        }
        mdio(b);
    }
}

#[test]
fn the_part_comes_up_under_the_drawn_address_and_says_so() {
    let _on = entropy(true);
    let bar = window();
    bar.present32(REG_RAL0 + 8, 0xDEAD_BEEF);
    bar.present32(REG_RAH0 + 8, RAH_AV | 0x1234);
    bar.present32(REG_MTA_BASE + 20, 0xFFFF_FFFF);
    let held = bmcr(BMCR_POWERED_DOWN);
    let _part = live(&bar, part(held.clone()));
    let mut mem = Memory::new();
    let Ok(d) = finish(mem.driver(&bar)) else { panic!("a working part comes up") };
    let mac = d.mac;
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    assert_eq!(bar.wrote32(REG_RAL0), u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]));
    assert_eq!(bar.wrote32(REG_RAH0), u32::from_le_bytes([mac[4], mac[5], 0, 0]) | RAH_AV);
    assert_eq!(bar.wrote32(REG_RAH0 + 8), 0, "old RAR1 entry gone");
    assert_eq!(bar.wrote32(REG_MTA_BASE + 20), 0, "MTA cleared");
    assert_eq!(held.load(std::sync::atomic::Ordering::SeqCst), 0x1340, "PHY powered up");
    assert_ne!(bar.wrote32(REG_CTRL) & CTRL_SLU, 0);
    assert_eq!(bar.wrote32(REG_RCTL) & 0x2, 0x2, "receiver on");
    assert_eq!(bar.wrote32(REG_TCTL) & 0x2, 0x2, "transmitter on");
    assert_eq!(bar.wrote32(REG_RDT), 31);
    assert_eq!(d.link, Some(LinkState { up: false, mbps: 10, full: false }));
    let text: Vec<String> = mac.iter().map(|b| format!("{b:02x}")).collect();
    let want = format!("igc: up 125c {} link=down\n", text.join(":"));
    assert_eq!(said().last(), Some(&want));
}
