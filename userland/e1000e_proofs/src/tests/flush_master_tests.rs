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

//! The I219 flush and Bus Master, on a part that has run on this driver's
//! rings: the flush masters only once both units are stopped and both rings
//! are this driver's, and runs the receiver with this driver's 2 KiB buffer
//! size, whatever RCTL holds; a broker that refuses Bus Master skips it.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{clear_log, logged, refuse_bus_master, seen, set_command, set_ring_status};
use nonos_libc::{watch, Step, MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use super::{helpers::started, probe::probe};
use crate::constants::regs::*;
use crate::constants::rxtx::{RCTL_BAM, RCTL_EN, RCTL_SECRC, TCTL_EN};
use crate::constants::Family;
use crate::init::flush_rings;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::pch;

/// Firmware's receiver: on, long frames, 16 KiB buffers (BSEX, BSIZE 01).
const FIRMWARE_RCTL: u32 = RCTL_EN | 1 << 5 | 1 << 25 | 1 << 16;

/// Rings firmware left pending, outside this driver's frames, and the claim
/// as the broker hands it over once it clears Bus Master there.
pub fn pending(bar: &FakeBar) {
    bar.present32(REG_TDLEN, 4096);
    bar.present32(REG_RCTL, FIRMWARE_RCTL);
    bar.present32(REG_RDBAL, 0x7FF0_0000);
    set_ring_status(0x100);
    set_command(MEM);
}

#[test]
fn the_flush_masters_only_on_this_drivers_rings_and_buffer_size() {
    let bar = pch();
    pending(&bar);
    watch(probe(&bar, &Arc::new(Behaviour::default())));
    flush_rings::run(&started(Memory::new().driver(&bar, Family::PchCnp, 0x15BB)));
    let on: Vec<_> = seen().into_iter().filter(|s| s.step == Step::Command(MEM | BM)).collect();
    assert_eq!(on.len(), 1, "{:?}", seen());
    let p = on[0].part;
    assert!(!p.rx_on && !p.tx_on && p.rings_own, "mastered with {p:?}");
    assert_ne!(bar.wrote32(REG_TCTL) & TCTL_EN, 0, "the transmit flush ran");
    assert_eq!(bar.wrote32(REG_RCTL), RCTL_BAM | RCTL_SECRC, "2 KiB buffers, left off");
}

#[test]
fn a_refused_bus_master_skips_the_flush_by_name() {
    let bar = pch();
    pending(&bar);
    refuse_bus_master();
    clear_log();
    flush_rings::run(&started(Memory::new().driver(&bar, Family::PchCnp, 0x15BB)));
    assert_eq!(bar.wrote32(REG_TDT), 0, "no descriptor handed over");
    assert_eq!(bar.wrote32(REG_TCTL) & TCTL_EN, 0);
    let refused = "e1000e: pci command write refused (bus master)";
    assert_eq!(logged().last().map(String::as_str), Some(refused));
}
