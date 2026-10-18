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

//! SRAT's affinity entries and SLIT's matrix: entries of every kind and of
//! none, enabled or not, lengths that lie; locality counts past the cap and
//! past the bytes, and distances that break the rules.

use super::gen::{damage, honest, sdt, Rng};
use super::sweep::{body, sweep};
use crate::arm_acpi::numa::slit_matrix::{matrix, MAX_LOCALITIES};
use crate::arm_acpi::numa::srat_entry::{entries, MAX_ENTRIES};

const SRAT: [(u8, usize); 6] = [(0, 14), (1, 38), (2, 22), (3, 16), (5, 30), (4, 8)];

#[test]
fn srat_entries_load_or_are_skipped() {
    sweep(
        |r| damage(sdt(b"SRAT", 3, &body(r, 12, &SRAT)), r),
        |t| Some(entries(t)).filter(|e| !e.is_empty()),
        |e, t| e.len() <= MAX_ENTRIES && 48 + 16 * e.len() <= t.len(),
    );
}

#[test]
fn srat_stops_at_its_cap() {
    let apic = honest(0, &[0, 7, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let t = sdt(b"SRAT", 3, &[vec![0; 12], apic.repeat(MAX_ENTRIES + 5)].concat());
    assert_eq!(entries(&t).len(), MAX_ENTRIES);
}

/// `n` localities, each a sound distance; when `hostile`, sometimes one
/// cell broken or the count a lie.
fn slit(r: &mut Rng, n: usize, hostile: bool) -> Vec<u8> {
    let cell = |i: usize, r: &mut Rng| if i / n == i % n { 10 } else { 11 + r.below(245) as u8 };
    let mut m: Vec<u8> = (0..n * n).map(|i| cell(i, r)).collect();
    if hostile && n > 0 && r.one_in(4) {
        let at = r.below(m.len());
        m[at] = r.next() as u8;
    }
    let count = if hostile && r.one_in(10) { r.next() } else { n as u64 };
    sdt(b"SLIT", 1, &[count.to_le_bytes().to_vec(), m].concat())
}

#[test]
fn slit_matrices_load_or_are_refused() {
    let make = |r: &mut Rng| {
        let n = r.pick(&[0, 1, 2, 3, 4, 9, MAX_LOCALITIES, MAX_LOCALITIES + 1]);
        damage(slit(r, n, true), r)
    };
    sweep(make, matrix, |(n, m), t| {
        *n <= MAX_LOCALITIES && m.len() == n * n && 44 + m.len() <= t.len()
    });
}

#[test]
fn slit_refuses_past_its_cap() {
    let r = &mut Rng::new(98);
    assert_eq!(matrix(&slit(r, MAX_LOCALITIES, false)).map(|m| m.0), Some(MAX_LOCALITIES));
    assert_eq!(matrix(&slit(r, MAX_LOCALITIES + 1, false)), None);
}
