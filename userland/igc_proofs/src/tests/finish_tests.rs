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

//! A bring-up that stops: the step is logged as "igc: <step>", the part is
//! stood down so no queue can DMA into ring memory, and every grant and the
//! claim go back in reverse order.

use nonos_libc::{entropy, given_back, said};

use crate::constants::regs::*;
use crate::init::finish;
use crate::init::reset::master::MASTER_STUCK;

use super::memory::Memory;
use super::model::window;

const EVERYTHING: [(&str, u64); 6] = [
    ("dma_unmap", 24),
    ("dma_unmap", 23),
    ("dma_unmap", 22),
    ("dma_unmap", 21),
    ("mmio_unmap", 20),
    ("device_release", 9),
];

#[test]
fn a_stuck_master_is_logged_and_everything_given_back() {
    let bar = window();
    bar.present32(REG_STATUS, 1 << 19);
    // Queues a previous owner left running, which must not outlive the grants.
    bar.present32(REG_RCTL, 0x0400_8002);
    bar.present32(REG_RXDCTL, 1 << 25);
    bar.present32(REG_TXDCTL, 1 << 25);
    let before = given_back().len();
    let mut mem = Memory::new();
    assert_eq!(finish(mem.driver(&bar)).err(), Some(MASTER_STUCK));
    assert_eq!(said().last(), Some(&format!("igc: {MASTER_STUCK}\n")));
    assert_eq!(bar.wrote32(REG_RCTL), 0, "stood down: receiver off");
    assert_eq!((bar.wrote32(REG_RXDCTL), bar.wrote32(REG_TXDCTL)), (0, 0), "queues off");
    assert_eq!(bar.wrote32(REG_IMC), 0xFFFF_FFFF);
    assert_eq!(given_back()[before..], EVERYTHING);
}

#[test]
fn no_entropy_means_no_address_and_no_enable_ever_written() {
    let _off = entropy(false);
    let bar = window();
    bar.present32(REG_EECD, 1 << 9);
    let before = given_back().len();
    let mut mem = Memory::new();
    assert_eq!(finish(mem.driver(&bar)).err(), Some("no entropy for station address"));
    assert_eq!(bar.wrote32(REG_RAL0), 0, "nothing in the receive filter");
    assert_eq!(bar.wrote32(REG_RCTL) & 0x2, 0, "receiver never enabled");
    assert_eq!(given_back()[before..], EVERYTHING);
}
