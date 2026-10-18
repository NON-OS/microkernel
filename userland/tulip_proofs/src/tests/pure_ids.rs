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

//! Which functions are taken, and the 32-bit bound on every grant.

use nonos_libc::{Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::discover::is_match;
use crate::setup::grant::fits_32;

#[test]
fn both_tulips_with_a_memory_window_are_taken() {
    let rec = |device: u16, kind: u8| {
        let mut r = DeviceRecord {
            device_id: 1,
            bus_kind: BUS_KIND_PCI,
            pci_class: 2,
            vendor: 0x1011,
            device,
            bar_count: 2,
            ..Default::default()
        };
        r.bars[1] = Bar { kind, size: 0x80 };
        r
    };
    assert!(is_match(&rec(0x0009, BAR_KIND_MMIO)) && is_match(&rec(0x0019, BAR_KIND_MMIO)));
    assert!(!is_match(&rec(0x0002, BAR_KIND_MMIO)), "the 21040 has another CSR layout");
    assert!(!is_match(&rec(0x0019, BAR_KIND_PIO)));
}

#[test]
fn a_grant_must_end_at_or_below_4_gib() {
    assert!(fits_32(0, 1 << 32));
    assert!(!fits_32(0xFFFF_F000, 0x2000));
}
