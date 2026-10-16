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

//! Bus mastering refused, or asked for by a part the reset did not stop:
//! either ends the attempt by name, with the part never mastering a ring.

use nonos_libc::{entropy, refuse_bus_master, seen, set_command, watch};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::live::{live_part, live_with};
use super::memory::Memory;
use super::model::window;
use super::order::released_quiet;
use super::probe::probe;
use crate::constants::regs::REG_RCTL;
use crate::constants::status::RCTL_EN;
use crate::init::finish;

/// A broker that refuses Bus Master ends the attempt by name, and the part,
/// left mastering since boot, is stopped and taken off the bus before release.
#[test]
fn a_refused_bus_master_ends_the_attempt_with_the_engines_never_on() {
    let _turn = entropy(true);
    let bar = window();
    let part = live_part(&bar);
    set_command(MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER);
    refuse_bus_master();
    watch(probe(&bar, &part));
    let err = finish(Memory::new().driver(&bar)).err();
    assert_eq!(err, Some("pci command write refused (bus master)"));
    assert!(seen().iter().all(|s| !s.part.rx_on && !s.part.tx_on), "{:?}", seen());
    released_quiet(&seen(), 2);
}

/// A part whose receiver is still on after the reset is not given Bus Master.
#[test]
fn a_receiver_that_survives_the_reset_is_named_and_never_mastered() {
    let _turn = entropy(true);
    let bar = window();
    let part = live_with(&bar, |b| b.present32(REG_RCTL, b.wrote32(REG_RCTL) | RCTL_EN));
    set_command(MK_PCI_CMD_MEMORY_SPACE);
    watch(probe(&bar, &part));
    let err = finish(Memory::new().driver(&bar)).err();
    assert_eq!(err, Some("RCTL/TCTL still on after reset"));
    assert!(seen().iter().all(|s| !s.bus_master), "{:?}", seen());
}
