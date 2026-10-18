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

//! The legacy handshake's steps over a register window in memory.

use nonos_devmodel::FakeBar;

use super::model::view;
use crate::constants::{
    LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_QUEUE_PFN, LEG_STATUS, STATUS_FAILED,
};
use crate::init::{capacity, negotiate, program};
use crate::regs::Regs;

fn window() -> (FakeBar, Regs) {
    let bar = FakeBar::new(64);
    let regs = Regs::mmio(view(&bar));
    (bar, regs)
}

// FLUSH alone: DISCARD (13), WRITE_ZEROES (14), MQ (12), RO (5), BLK_SIZE
// (6) and TOPOLOGY (10) are offered here and must not be claimed.
#[test]
fn only_flush_is_ever_claimed() {
    for (host, claimed) in [(u32::MAX, 1 << 9), (!(1 << 9), 0), (0, 0)] {
        let (bar, regs) = window();
        bar.present32(LEG_HOST_FEATURES, host);
        negotiate(regs).expect("features taken");
        assert_eq!(bar.wrote32(LEG_GUEST_FEATURES), claimed, "host offered {host:#x}");
    }
}

#[test]
fn the_ring_is_given_by_page_number_and_refused_above_the_legacy_range() {
    let (bar, regs) = window();
    program(regs, 0x1234_5000, false).expect("in range");
    assert_eq!(bar.wrote32(LEG_QUEUE_PFN), 0x1_2345);
    let (bar, regs) = window();
    assert!(program(regs, 1 << 44, false).is_err());
    assert_eq!((bar.wrote32(LEG_QUEUE_PFN), bar.wrote8(LEG_STATUS)), (0, STATUS_FAILED));
}

#[test]
fn with_msix_the_queue_raises_entry_0_and_the_configuration_moves_on_four() {
    let (bar, regs) = window();
    program(regs, 0x1000, true).expect("in range");
    assert_eq!((bar.wrote16(0x14), bar.wrote16(0x16)), (0xFFFF, 0), "config none, queue 0");
    let (bar, regs) = window();
    bar.present32(0x18, 0x10);
    bar.present32(0x1C, 0x2);
    assert_eq!(capacity(regs, true), Ok(0x2_0000_0010));
    let (bar, regs) = window();
    bar.present32(0x14, 0x10);
    assert_eq!(capacity(regs, false), Ok(0x10));
}

#[test]
fn a_disk_of_no_sectors_is_refused_and_the_device_told() {
    let (bar, regs) = window();
    assert_eq!(capacity(regs, false), Err("virtio-blk: zero capacity"));
    assert_eq!(bar.wrote8(LEG_STATUS), STATUS_FAILED);
}
