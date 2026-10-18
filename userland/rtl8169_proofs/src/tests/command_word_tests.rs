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

//! The PCI Command word against the broker's allowlist rule, as it is today
//! and as it will be once the broker stores the word as written.

use crate::setup::command_word;

const IO: u16 = 1 << 0;
const MEM: u16 = 1 << 1;
const BM: u16 = 1 << 2;
const SERR: u16 = 1 << 8;
const INTX_OFF: u16 = 1 << 10;
const WRITABLE: u16 = MEM | BM | INTX_OFF;

// The rule src/hardware/broker/pci/command.rs validate_command applies: a
// value inside the allowlist is ORed onto the live register, any other is
// taken verbatim and refused if it changes a bit outside the allowlist.
fn broker(new: u16, current: u16) -> Option<u16> {
    let desired = if new & !WRITABLE == 0 { current | new } else { new };
    ((desired ^ current) & !WRITABLE == 0).then_some(desired)
}

// The rule once fixed: the allowlisted bits take the value as written.
fn stored(new: u16, current: u16) -> u16 {
    (current & !WRITABLE) | (new & WRITABLE)
}

#[test]
fn every_firmware_state_is_accepted_and_bus_master_set_when_asked() {
    for current in 0..=u16::MAX {
        let set = broker(command_word(true), current).expect("the broker refused the word");
        assert_eq!(set & (MEM | BM), MEM | BM);
        assert_eq!(set & !WRITABLE, current & !WRITABLE, "no bit outside the allowlist moved");
        assert_eq!(stored(command_word(true), current) & (MEM | BM), MEM | BM);
    }
}

#[test]
fn the_word_without_bus_master_turns_it_off_once_stored_as_written() {
    for current in 0..=u16::MAX {
        let word = command_word(false);
        assert_eq!(word & !WRITABLE, 0, "only allowlisted bits are ever written");
        assert!(broker(word, current).is_some(), "never refused, whatever firmware left");
        assert_eq!(stored(word, current) & (MEM | BM), MEM);
    }
}

// The machine that refused the old word: a port and an MMIO BAR in the
// record, firmware with I/O decode off and SERR on.
#[test]
fn firmware_with_io_decode_off_and_serr_on_never_refuses_the_claim() {
    let current = MEM | SERR;
    assert_eq!(broker(IO | MEM | BM, current), None);
    assert_eq!(broker(command_word(true), current), Some(MEM | BM | SERR));
}
