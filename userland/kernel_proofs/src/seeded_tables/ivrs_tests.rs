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
//! IVRS in the seeded sweep: headed tables whose blocks and device entries
//! lie, through the kernel's header check and its pure walks. The IOMMUs
//! found are exactly the distinct non-zero bases of the whole IVHD blocks in
//! order, the walk complete only when the blocks end cleanly and all fit;
//! every span, alias, special and reserved range holds its cap and rules.

use std::cell::Cell;

use super::gen::{damage, sdt, Rng};
use super::ivrs_gen::ivrs_body;
use super::ivrs_read::{blocks, read, Seen};
use super::sweep::sweep;
use crate::proofs::proofs_2::firmware_iommu::ivrs_walk::MAX_AMD_IOMMUS;

/// The bases a walk of whole IVHD blocks must find, and whether all fit.
fn expected(t: &[u8]) -> (Vec<u64>, bool) {
    let n = u32::from_le_bytes(t[4..8].try_into().unwrap()) as usize;
    let (list, clean) = blocks(&t[..n]);
    let mut bases = Vec::new();
    for &(at, kind, length) in &list {
        let base = t.get(at + 8..at + 16).map_or(0, |b| u64::from_le_bytes(b.try_into().unwrap()));
        let ivhd = matches!(kind, 0x10 | 0x11 | 0x40) && length >= 24;
        if ivhd && base != 0 && !bases.contains(&base) {
            if bases.len() == MAX_AMD_IOMMUS {
                return (bases, false);
            }
            bases.push(base);
        }
    }
    (bases, clean)
}

fn sound(s: &Seen) -> bool {
    let page = |a: u64| a & 0xFFF == 0;
    s.spans.iter().all(|p| p.first <= p.last)
        && s.aliases.iter().all(|a| a.first <= a.last)
        && s.specials.iter().enumerate().all(|(i, id)| !s.specials[..i].contains(id))
        && s.reserved.iter().all(|r| {
            r.first <= r.last
                && r.start < r.end
                && page(r.start)
                && page(r.end)
                && (r.read || r.write)
        })
}

#[test]
fn ivrs_blocks_and_entries_stay_inside_the_table_and_hold_their_caps() {
    let make = |r: &mut Rng| damage(sdt(b"IVRS", 2, &ivrs_body(r)), r);
    let kept = Cell::new(0u8);
    sweep(make, read, |s, t| {
        let lists =
            [s.bases.len(), s.spans.len(), s.aliases.len(), s.specials.len(), s.reserved.len()];
        let mask = lists.iter().enumerate().fold(0, |m, (i, &n)| m | ((n > 0) as u8) << i);
        kept.set(kept.get() | mask);
        (s.bases.clone(), s.complete) == expected(t) && sound(s)
    });
    assert_eq!(kept.get(), 0b1_1111, "a base, span, alias, special and reserved range each kept");
}
