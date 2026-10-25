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

//! an IO-APIC entry in remappable format, against Linux's
//! IR_IO_APIC_route_entry: vector 7:0, index2 (handle bit 15) 11, polarity
//! 13, trigger 15, mask 16, format 48, index (handle 14:0) 63:49.

use super::ioapic_rte::{handle_of, remappable};

#[test]
fn an_entry_matches_the_linux_layout() {
    let (low, high) = remappable(0x8123, 0x91, true, true);
    assert_eq!(low, 0x91 | 1 << 11 | 1 << 13 | 1 << 15 | 1 << 16);
    assert_eq!(high, 1 << 16 | 0x0123 << 17);
    let (low, high) = remappable(5, 0x81, false, false);
    assert_eq!((low, high), (0x81 | 1 << 16, 1 << 16 | 5 << 17), "edge, active high");
}

#[test]
fn every_handle_reads_back_and_is_written_masked() {
    for handle in 0..=u16::MAX {
        let (low, high) = remappable(handle, (handle as u8) | 0x80, handle & 1 == 0, false);
        assert_eq!(handle_of(low, high), Some(handle));
        assert_ne!(low & 1 << 16, 0, "masked until the driver acks");
        assert_eq!(low & 0x7FF, (handle as u32 & 0xFF) | 0x80, "vector, and bits 10:8 zero");
    }
}

#[test]
fn a_compatibility_format_entry_names_no_handle() {
    // Fixed delivery, physical, vector 0x81 to APIC id 3: the format bit clear.
    assert_eq!(handle_of(0x81, 3 << 24), None);
    assert_eq!(handle_of(0x81 | 1 << 11, 0xFFFE_0000), None);
}
