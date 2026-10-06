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

//! A refused ring status read, and the quiesce before every reset.

use crate::constants::regs::*;
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::constants::Family;
use crate::init::{flush_rings, quiesce};
use crate::model::memory::Memory;
use crate::model::window::{i82574, pch};
use nonos_libc::{clear_log, config_reads, logged, set_ring_status};

#[test]
fn a_refused_config_read_is_logged_and_the_flush_skipped() {
    let bar = pch();
    bar.present32(REG_TDLEN, 4096);
    set_ring_status(-22);
    clear_log();
    let mut mem = Memory::new();
    flush_rings::run(&mem.driver(&bar, Family::PchSpt, 0x156F));
    assert_eq!(bar.wrote32(REG_TDT), 0);
    let line = "e1000e: DESC_RING_STATUS config read refused, ring flush skipped";
    assert_eq!(logged(), [line]);
}

#[test]
fn quiesce_flushes_on_spt_and_later_only_and_stops_both_units() {
    let (lpt, old) = (pch(), i82574());
    let mut mem = Memory::new();
    quiesce::run(&mem.driver(&lpt, Family::PchLpt, 0x153A));
    quiesce::run(&mem.driver(&old, Family::I82574, 0x10D3));
    assert!(config_reads().is_empty(), "no ring status read before pch_spt");
    for bar in [lpt, old] {
        assert_eq!(bar.wrote32(REG_IMC), 0xFFFF_FFFF);
        assert_eq!(bar.wrote32(REG_RCTL) & RCTL_EN, 0);
        assert_eq!(bar.wrote32(REG_TCTL) & TCTL_EN, 0);
    }
}
