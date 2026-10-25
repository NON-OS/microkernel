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

//! DGON, the device-gone notice: what the kernel builds a driver reads back,
//! and nothing else is one. A depart's IRQ grants are remembered as gone,
//! for that pid alone, the oldest dropped past the bound.

use super::acpi::dgon_wire::*;
use super::irq_gone::{departed, mark};

#[test]
fn the_notice_reads_back_and_nothing_else_is_one() {
    let d = Dgon { why: WHY_GONE, device_id: 0x1_0000_0003 };
    let b = dgon_encode(d);
    assert_eq!(&b[..4], b"DGON");
    assert_eq!(dgon_parse(&b, 0), Some(d));
    assert_eq!(dgon_parse(&b, 41), None, "only sender 0 speaks for the kernel");
    assert_eq!(dgon_parse(&b[..15], 0), None, "short");
    assert_eq!(dgon_parse(&[&b[..], &[0]].concat(), 0), None, "long");
    let with = |i: usize, v: u8| {
        let mut m = b;
        m[i] = v;
        m
    };
    assert_eq!(dgon_parse(&with(0, b'X'), 0), None, "wrong tag");
    assert_eq!(dgon_parse(&with(4, 2), 0), None, "a version this reader does not know");
    assert_eq!(dgon_parse(&with(6, 9), 0), None, "a reason this version does not define");
    assert_eq!(dgon_parse(&[], 0), None);
}

#[test]
fn departed_grants_are_remembered_per_pid_and_bounded() {
    mark(7, 100);
    assert!(departed(7, 100));
    assert!(!departed(8, 100), "another pid's grant of the same id");
    assert!(!departed(7, 101));
    for g in 0..256u64 {
        mark(9, 1000 + g);
    }
    assert!(!departed(7, 100), "the oldest went past the bound");
    assert!(departed(9, 1000) && departed(9, 1255));
}
