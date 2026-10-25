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

//! The rules that need no controller: which functions are taken, the
//! 32-bit bound, the descriptor geometry and the command bits.

use nonos_libc::{Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::discover::is_match;

fn record(device: u16, nam: (u8, u64), nabm: (u8, u64)) -> DeviceRecord {
    let mut r = DeviceRecord {
        device_id: 1,
        bus_kind: BUS_KIND_PCI,
        vendor: 0x8086,
        device,
        bar_count: 2,
        ..Default::default()
    };
    r.bars[0] = Bar { kind: nam.0, size: nam.1 };
    r.bars[1] = Bar { kind: nabm.0, size: nabm.1 };
    r
}

#[test]
fn only_the_ich_ac97_with_both_windows_in_io_space_is_taken() {
    let io = |size| (BAR_KIND_PIO, size);
    assert!(is_match(&record(0x2415, io(256), io(64))));
    assert!(!is_match(&record(0x2425, io(256), io(64))), "another ICH");
    assert!(!is_match(&record(0x2415, (BAR_KIND_MMIO, 256), io(64))));
    assert!(!is_match(&record(0x2415, io(0x40), io(64))), "no room for the vendor id");
    assert!(!is_match(&record(0x2415, io(256), io(0x20))), "no room for the semaphore");
    let mut one_bar = record(0x2415, io(256), io(64));
    one_bar.bar_count = 1;
    assert!(!is_match(&one_bar));
}
