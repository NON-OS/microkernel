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

//! A unit that still reads on after the stop before the I219 flush on a
//! started part: Bus Master is never turned on for it, and the flush is
//! skipped by name.

use std::sync::Arc;

use nonos_libc::{clear_log, command, logged, seen, watch, MK_PCI_CMD_BUS_MASTER as BM};

use super::flush_master_tests::pending;
use super::helpers::started;
use super::probe::probe;
use crate::constants::regs::{REG_RCTL, REG_TDT};
use crate::constants::rxtx::RCTL_EN;
use crate::constants::Family;
use crate::init::flush_rings;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::pch;
use crate::regs::on_read;

#[test]
fn a_receiver_that_reads_on_after_the_stop_is_never_mastered() {
    let bar = pch();
    pending(&bar);
    watch(probe(&bar, &Arc::new(Behaviour::default())));
    clear_log();
    let _rctl = on_read(REG_RCTL, |v| v | RCTL_EN);
    flush_rings::run(&started(Memory::new().driver(&bar, Family::PchCnp, 0x15BB)));
    let skipped = "e1000e: RCTL/TCTL still on after the stop, ring flush skipped";
    assert_eq!(logged().last().map(String::as_str), Some(skipped));
    assert!(seen().iter().all(|s| !s.bus_master) && command() & BM == 0, "{:?}", seen());
    assert_eq!(bar.wrote32(REG_TDT), 0, "no descriptor handed over");
}
