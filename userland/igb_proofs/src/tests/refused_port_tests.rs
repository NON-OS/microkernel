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
//! A port this driver does not drive, carried through a whole attempt: the
//! reset runs, the link mode it loads refuses the port by name, and nothing
//! is enabled, no address drawn, and every grant goes back.

use std::sync::atomic::Ordering;

use nonos_libc::{entropy, given_back, said};

use crate::bring::finish;
use crate::bring::port::SERDES;
use crate::facts::regs::*;

use super::memory::Memory;
use super::model::{live, window};
use super::part_model::part;
use super::phy_model::{bmcr, BMCR_POWERED_DOWN};

#[test]
fn a_serdes_port_is_refused_by_name_and_left_as_it_was_found() {
    let _on = entropy(true);
    let bar = window();
    bar.present32(REG_CTRL_EXT, 0x00C0_0000);
    let held = bmcr(BMCR_POWERED_DOWN);
    let _part = live(&bar, part(held.clone(), None));
    let before = given_back().len();
    let mut mem = Memory::new();
    assert_eq!(finish(mem.driver(&bar)).err(), Some(SERDES));
    assert_eq!(said().last(), Some(&format!("igb: {SERDES}\n")));
    assert_eq!(bar.wrote32(ral(0)), 0, "no address drawn into the filter");
    assert_eq!(bar.wrote32(REG_RCTL) & 0x2, 0, "receiver never enabled");
    assert_eq!(bar.wrote32(REG_TCTL) & 0x2, 0, "transmitter never enabled");
    assert_eq!(held.load(Ordering::SeqCst), BMCR_POWERED_DOWN, "PHY never touched");
    assert_eq!(given_back().len() - before, 6, "four DMA grants, the window and the claim");
}
