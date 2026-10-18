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
//! that turns Bus Master on; a queue still on after the reset ends the
//! attempt by name, and Bus Master never goes on for it.

use std::sync::{atomic::Ordering::SeqCst, Arc};

use nonos_libc::MK_PCI_CMD_BUS_MASTER as BM;
use nonos_libc::{command, entropy, given_back, on_clock, seen, Seen, Step};

use super::bus_master_tests::claimed;
use super::memory::Memory;
use super::order::bus_master_after_reset_and_rings;
use super::probe::probe;
use super::reset_queue_tests::{written_off, QUEUES, STILL_ON};
use crate::constants::regs::{REG_RCTL, REG_RDBAL, REG_RXDCTL, REG_TCTL, REG_TDBAL};
use crate::constants::rx_bits::{RCTL_EN, RXDCTL_QUEUE_ENABLE};
use crate::constants::tx_bits::TCTL_EN;
use crate::init::finish;

#[test]
fn a_part_left_running_is_stopped_and_reset_before_bus_master_goes_on() {
    let _turn = entropy(true);
    let (bar, _part, resets) = claimed(|_| {});
    for (reg, value) in QUEUES.into_iter().chain([(REG_RCTL, RCTL_EN), (REG_TCTL, TCTL_EN)]) {
        bar.present32(reg, value);
    }
    bar.present32(REG_RDBAL, 0x7FF0_0000);
    bar.present32(REG_TDBAL, 0x7FF1_0000);
    let left = probe(&bar, &resets)();
    assert!(left.rx_on && left.tx_on && !left.rings_own, "{left:?}");
    finish(Memory::new().driver(&bar)).expect("brought up");
    bus_master_after_reset_and_rings(&seen(), resets.load(SeqCst));
}

#[test]
fn a_queue_still_on_after_the_reset_is_named_and_never_mastered() {
    let _turn = entropy(true);
    let (bar, _part, _resets) = claimed(|_| {});
    // Back on at each look once the driver has written it off after a reset.
    let b = Arc::clone(&bar);
    let _clock = on_clock(move || {
        if written_off(&b) {
            b.present32(REG_RXDCTL, RXDCTL_QUEUE_ENABLE);
        }
    });
    let mark = given_back().len();
    assert_eq!(finish(Memory::new().driver(&bar)).err(), STILL_ON.err());
    let on = |s: &Seen| matches!(s.step, Step::Command(v) if v & BM != 0);
    assert!(!seen().iter().any(on) && command() & BM == 0, "{:?}", seen());
    assert_eq!(given_back()[mark..], [("device_release", 9)], "and the device went back");
}
