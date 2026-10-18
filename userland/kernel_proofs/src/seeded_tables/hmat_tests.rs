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

//! HMAT's structures: each table loads what fits and no more than its caps.

use super::gen::sdt;
use super::hmat_gen::{hmat, structure, MAX_DOMAINS};
use super::sweep::sweep;
use crate::arm_acpi::numa::hmat_entry::{structures, MAX_STRUCTURES};

#[test]
fn hmat_structures_load_or_are_skipped() {
    let kept = |t: &[u8]| Some(structures(t)).filter(|h| h.attributes.len() + h.matrices.len() > 0);
    sweep(hmat, kept, |h, t| {
        let sound = |m: &crate::arm_acpi::numa::hmat_entry::Matrix| {
            let (i, j) = (m.initiators.len(), m.targets.len());
            i <= MAX_DOMAINS && j <= MAX_DOMAINS && m.entries.len() == i * j
        };
        h.attributes.len() + h.matrices.len() <= MAX_STRUCTURES.min(t.len() / 8)
            && h.matrices.iter().all(sound)
    });
}

#[test]
fn hmat_stops_at_its_caps() {
    let one = |s: Vec<u8>| structures(&sdt(b"HMAT", 2, &[vec![0; 4], s].concat()));
    let h = one(structure(0, &[0; 32]).repeat(MAX_STRUCTURES + 9));
    assert_eq!(h.attributes.len(), MAX_STRUCTURES);
    let wide = |m: usize| {
        let head = [0, 0, 0, 0, m as u8, (m >> 8) as u8, 0, 0, 1, 0, 0, 0];
        structure(1, &[&head[..], &vec![0; 12 + 4 * (m + 1) + 2 * m]].concat())
    };
    assert_eq!(one(wide(MAX_DOMAINS)).matrices[0].initiators.len(), MAX_DOMAINS);
    assert!(one(wide(MAX_DOMAINS + 1)).matrices.is_empty(), "past the cap");
}

#[test]
fn hmat_ends_at_a_structure_shorter_than_its_header() {
    // Read on by its length 4, the walk would meet a type 4 of length 8
    // and then the attributes; it must stop at the short one instead.
    let short = [9, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 0];
    let t = sdt(b"HMAT", 2, &[&[0; 4][..], &short, &structure(0, &[0; 32])].concat());
    assert!(structures(&t).attributes.is_empty());
    let whole = sdt(b"HMAT", 2, &[&[0; 4][..], &structure(0, &[0; 32])].concat());
    assert_eq!(structures(&whole).attributes.len(), 1);
}
