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

//! The end of an attempt. One that works prints a single line naming the
//! part and its link, never the drawn address; one that fails stands the
//! part down (receiver off, engines stopped, a chip reset) and gives every
//! grant back, newest first, before it reports the step that stopped it.

use std::sync::{Arc, Mutex};

use nonos_libc::{entropy, given_back, said};

use crate::constants::mac::{MAC_RX_MODE, MAC_TX_MODE, RX_MODE_ENABLE, TX_MODE_ENABLE};
use crate::init::finish;
use crate::init::station_address::NO_ENTROPY;

use super::build::{LOM, REV_57766};
use super::config_replay::{replay_against_the_broker, seed, seeded};
use super::memory::Memory;
use super::model::{live, window};
use super::phy_model::phy;
use super::phy_states::linked;

#[test]
fn a_working_attempt_says_up_without_the_address() {
    let bar = window();
    let mut m = Memory::new();
    seed();
    let _part = live(&bar, phy(Arc::new(Mutex::new(linked(0x0700)))));
    let Ok(d) = finish(m.driver(&bar, REV_57766, LOM)) else { panic!("not up") };
    let up: Vec<String> = said().into_iter().filter(|l| l.contains(" up ")).collect();
    assert_eq!(up, ["tg3: up 16b4 rev 57766001 link=up\n"]);
    let mac: String = d.mac.iter().map(|b| format!("{b:02x}")).collect();
    assert!(said().iter().all(|l| !l.replace(':', "").contains(&mac)));
}

#[test]
fn a_failed_attempt_stands_down_and_gives_every_grant_back() {
    let bar = window();
    let mut m = Memory::new();
    seed();
    let start = seeded();
    let _part = live(&bar, phy(Arc::new(Mutex::new(linked(0x0700)))));
    let _off = entropy(false);
    let err = finish(m.driver(&bar, REV_57766, LOM)).err();
    assert_eq!(err, Some(NO_ENTROPY));
    assert_eq!(bar.wrote32(MAC_RX_MODE) & RX_MODE_ENABLE, 0);
    assert_eq!(bar.wrote32(MAC_TX_MODE) & TX_MODE_ENABLE, 0);
    let back = [
        ("dma_unmap", 23),
        ("dma_unmap", 22),
        ("dma_unmap", 21),
        ("mmio_unmap", 20),
        ("device_release", 9),
    ];
    assert_eq!(given_back(), back);
    assert!(said().contains(&format!("tg3: {NO_ENTROPY}\n")));
    // The stand-down's own chip reset is held to the broker's rule too.
    replay_against_the_broker(&start);
}
