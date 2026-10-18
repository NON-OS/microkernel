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

//! A part a dead predecessor left running: both engines on, both rings
//! naming its frames, and Bus Master on since boot. The bring-up stops and
//! resets it, and sees both engines off, before the first Command write
//! that turns Bus Master on; a receiver still on after the reset is named,
//! and Bus Master never goes on for it.

use nonos_libc::{command, entropy, seen, Seen, Step, MK_PCI_CMD_BUS_MASTER as BM};

use super::bus_master_tests::{claimed, up};
use super::order::bus_master_after_reset_and_rings;
use super::probe::{probe, resets};
use crate::constants::regs::{REG_RCTL, REG_RDBAL, REG_TCTL, REG_TDBAL};
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::model::memory::Memory;
use crate::regs::on_read;

#[test]
fn a_part_left_running_is_stopped_and_reset_before_bus_master_goes_on() {
    let _turn = entropy(true);
    let (bar, part) = claimed();
    for (reg, value) in [(REG_RCTL, RCTL_EN), (REG_TCTL, TCTL_EN)] {
        bar.present32(reg, value);
    }
    bar.present32(REG_RDBAL, 0x7FF0_0000);
    bar.present32(REG_TDBAL, 0x7FF1_0000);
    let left = probe(&bar, &part.how)();
    assert!(left.rx_on && left.tx_on && !left.rings_own, "{left:?}");
    up(&bar, &mut Memory::new()).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets(&part.how));
}

#[test]
fn a_receiver_still_on_after_the_reset_is_named_and_never_mastered() {
    let _turn = entropy(true);
    let (bar, _part) = claimed();
    let _rctl = on_read(REG_RCTL, |v| v | RCTL_EN);
    let err = up(&bar, &mut Memory::new()).err();
    assert_eq!(err, Some("RCTL/TCTL still on after reset"));
    let on = |s: &Seen| matches!(s.step, Step::Command(v) if v & BM != 0);
    assert!(!seen().iter().any(on) && command() & BM == 0, "{:?}", seen());
}
