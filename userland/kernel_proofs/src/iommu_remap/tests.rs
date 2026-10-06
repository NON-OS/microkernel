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

use super::irte::{destination, encode, is_present, source, vector, Route};
use super::regs::{irta_entries, irta_value, IRTA_EIME, IRTE_ENTRIES};

const XHCI: u16 = 0x14 << 3;

#[test]
fn irta_names_one_page_of_entries() {
    let value = irta_value(0x1234_5000, false);
    assert_eq!(value, 0x1234_5007);
    assert_eq!(irta_entries(value), IRTE_ENTRIES as u32);
    assert_eq!(irta_value(0x1234_5000, true), 0x1234_5007 | IRTA_EIME);
    assert_eq!(IRTA_EIME, 1 << 11);
}

/* Linux: present bit 0, trigger mode bit 4, vector bits 23:16, dest_id bits
63:32 holding IRTE_DEST(d) = d << 8 in xAPIC mode; svt bits 83:82 = 01,
sq bits 81:80 = 00, sid bits 79:64. */
#[test]
fn an_xapic_entry_matches_the_linux_layout() {
    let route = Route { vector: 0x41, destination: 3, source: XHCI, level: false };
    let [low, high] = encode(route, false).unwrap();
    assert_eq!(low, 1 | (0x41 << 16) | (3 << 40));
    assert_eq!(high, (1 << 18) | XHCI as u64);
    let level = encode(Route { level: true, ..route }, false).unwrap();
    assert_eq!(level[0], low | (1 << 4));
}

#[test]
fn a_destination_is_never_truncated() {
    let wide = Route { vector: 0x30, destination: 0x100, source: 0, level: false };
    assert_eq!(encode(wide, false), None, "xAPIC entry holds eight bits");
    let entry = encode(wide, true).unwrap();
    assert_eq!(destination(entry, true), 0x100);
    let edge = Route { destination: 0xFF, ..wide };
    assert_eq!(destination(encode(edge, false).unwrap(), false), 0xFF);
}

#[test]
fn every_vector_and_source_reads_back() {
    for v in 0..=255u8 {
        for &s in &[0u16, XHCI, 0x0200, 0xFFFF] {
            let route = Route { vector: v, destination: (v as u32) & 0xF, source: s, level: false };
            let entry = encode(route, false).unwrap();
            assert!(is_present(entry));
            assert_eq!(vector(entry), v);
            assert_eq!(source(entry), s);
            assert_eq!(destination(entry, false), (v as u32) & 0xF);
        }
    }
}
