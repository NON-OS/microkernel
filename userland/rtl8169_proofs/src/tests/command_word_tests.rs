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

//! The PCI Command word against the broker's allowlist rule.

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

#[test]
fn every_record_and_firmware_state_is_accepted_with_bus_master_on() {
    for record in [0, IO, MEM, IO | MEM] {
        for current in 0..=u16::MAX {
            let word = command_word(record);
            let set = broker(word, current).expect("the broker refused the word");
            assert_ne!(set & BM, 0);
            assert_eq!(set & !WRITABLE, current & !WRITABLE);
        }
    }
}

// The machine that refused the old word: a port and an MMIO BAR in the
// record, firmware with I/O decode off and SERR on.
#[test]
fn firmware_with_io_decode_off_and_serr_on_no_longer_refuses_the_claim() {
    let record = IO | MEM;
    let current = MEM | SERR;
    assert_eq!(broker(record | BM, current), None);
    assert_eq!(broker(command_word(record), current), Some(MEM | BM | SERR));
}
