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

//! HMAT read from its bytes: attributes with and without an initiator, a
//! latency and a bandwidth matrix queried entry by entry (no value and
//! unreachable included), and structures that lie about their size.

use super::hmat_build::*;
use crate::arm_acpi::numa::hmat_entry::structures;
use crate::arm_acpi::numa::hmat_query::{query, HmatValue};

fn machine() -> Vec<u8> {
    hmat(&[
        attributes(Some(0), 0),
        attributes(None, 2),
        matrix(0, 0, 1000, &[0, 1], &[0, 2], &[80, 0, 140, 0xFFFF]),
        matrix(0, 3, 1, &[0], &[0, 2], &[20_000, 6_000]),
        matrix(1, 0, 1000, &[0], &[0], &[10]),
    ])
}

#[test]
fn attributes_and_matrices() {
    let h = structures(&machine());
    assert_eq!(h.attributes.len(), 2);
    assert_eq!((h.attributes[0].memory, h.attributes[0].initiator), (0, Some(0)));
    assert_eq!((h.attributes[1].memory, h.attributes[1].initiator), (2, None));
    let m = &h.matrices;
    assert_eq!(query(m, 0, 0, 0, 0), Some(HmatValue::Value(80_000)), "picoseconds");
    assert_eq!(query(m, 0, 0, 1, 0), Some(HmatValue::Value(140_000)));
    assert_eq!(query(m, 0, 0, 0, 2), None, "firmware gave no value");
    assert_eq!(query(m, 0, 0, 1, 2), Some(HmatValue::Unreachable));
    assert_eq!(query(m, 0, 3, 0, 2), Some(HmatValue::Value(6_000)), "MB/s");
    assert_eq!(query(m, 1, 0, 0, 0), Some(HmatValue::Value(10_000)), "a cache level");
    assert_eq!(query(m, 0, 1, 0, 0), None, "no read-latency matrix");
    assert_eq!(query(m, 0, 0, 5, 0), None, "no such initiator");
}

#[test]
fn bad_structures() {
    let mut cut = matrix(0, 0, 1, &[0, 1], &[0], &[1, 2]);
    cut.truncate(cut.len() - 2);
    let n = cut.len() as u32;
    cut[4..8].copy_from_slice(&n.to_le_bytes());
    assert!(structures(&hmat(&[cut])).matrices.is_empty(), "entries past the structure");
    let bad_kind = matrix(0, 6, 1, &[0], &[0], &[1]);
    assert!(structures(&hmat(&[bad_kind])).matrices.is_empty(), "data type 6");
    let mut overrun = hmat(&[attributes(Some(0), 0)]);
    overrun[44..48].copy_from_slice(&400u32.to_le_bytes());
    assert_eq!(structures(&overrun).attributes, [], "a length past the table");
    assert!(structures(&[]).matrices.is_empty());
}
