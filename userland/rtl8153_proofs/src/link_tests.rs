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

//! The link read on the clock, at most once a second, and kept.

use std::time::Duration;

use nonos_usbnet::Nic;

use crate::chip::{bound, reads_of, PLA};

const PHYSTATUS: u16 = 0xe908;

#[test]
fn the_link_is_read_at_most_once_a_second_and_link_up_never_reads_it() {
    let (mut nic, bus, chip) = bound(0x5c10);
    let mut out = [0u8; 1514];
    nic.recv(&mut out).unwrap();
    nic.recv(&mut out).unwrap();
    assert!(!nic.link_up());
    assert_eq!(reads_of(&bus, PHYSTATUS), 1);
    chip.borrow_mut().regs.put(PLA, PHYSTATUS, &0x0013u16.to_le_bytes());
    assert!(!nic.link_up(), "the kept value until the next look");
    std::thread::sleep(Duration::from_millis(1_050));
    nic.recv(&mut out).unwrap();
    assert!(nic.link_up());
    assert_eq!(reads_of(&bus, PHYSTATUS), 2);
    chip.borrow_mut().regs.put(PLA, PHYSTATUS, &0u16.to_le_bytes());
    std::thread::sleep(Duration::from_millis(1_050));
    nic.recv(&mut out).unwrap();
    assert!(!nic.link_up());
}
