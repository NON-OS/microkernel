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
//! What the bring-up does differently by MAC: an I350 clears all 32
//! receive addresses and Go Link Disconnect, and an I210 selects PHY page 0
//! before it touches BMCR.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_libc::entropy;

use crate::bring::finish;
use crate::facts::mac::*;
use crate::facts::parts::Part;
use crate::facts::regs::*;

use super::memory::Memory;
use super::model::{live, window};
use super::part_model::part;
use super::phy_model::{bmcr, BMCR_POWERED_DOWN};

#[test]
fn an_i350_clears_every_receive_address_and_go_link_disconnect() {
    let _on = entropy(true);
    let bar = window();
    bar.present32(ral(31) + 4, RAH_AV | 0x5678);
    bar.present32(REG_PHPM, PHPM_GO_LINKD | 0x1);
    let _part = live(&bar, part(bmcr(BMCR_POWERED_DOWN), None));
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    d.part = Part::I350;
    assert!(finish(d).is_ok());
    assert_eq!(ral(31), 0x54E0 + 15 * 8);
    assert_eq!(bar.wrote32(ral(31) + 4), 0, "RAR31, the I350's last, gone");
    assert_eq!(bar.wrote32(REG_PHPM), 0x1, "GO_LINKD off, the rest kept");
}

#[test]
fn an_i210_reaches_bmcr_on_page_0_whatever_page_was_left() {
    let _on = entropy(true);
    let bar = window();
    let held = bmcr(BMCR_POWERED_DOWN);
    let page = Arc::new(AtomicU32::new(3));
    let _part = live(&bar, part(held.clone(), Some(page.clone())));
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    d.part = Part::I210;
    d.pci_device = 0x1533;
    assert!(finish(d).is_ok());
    assert_eq!(held.load(Ordering::SeqCst), 0x1340, "BMCR itself powered up");
    assert_eq!(page.load(Ordering::SeqCst), 0);
}
