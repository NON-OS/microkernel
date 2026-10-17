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

//! An entry told wrong, its array CRC made good again: a partition that
//! shares a sector with another, runs past the usable range or the disk,
//! ends before it starts, or starts before the usable range.

use super::disk::Disk;
use super::lies::Lie;
use super::rng::Rng;

fn pick(r: &mut Rng, of: &[usize]) -> Option<usize> {
    (!of.is_empty()).then(|| of[r.range(0, of.len() as u64 - 1) as usize])
}

fn span(d: &mut Disk, w: usize, i: usize) -> (u64, u64) {
    let e = d.entry(w, i);
    (
        u64::from_le_bytes(e[32..40].try_into().unwrap()),
        u64::from_le_bytes(e[40..48].try_into().unwrap()),
    )
}

fn set(d: &mut Disk, w: usize, i: usize, first: u64, last: u64) {
    let e = d.entry(w, i);
    e[32..40].copy_from_slice(&first.to_le_bytes());
    e[40..48].copy_from_slice(&last.to_le_bytes());
}

pub fn entry_lie(r: &mut Rng, lie: Lie, w: usize, d: &mut Disk) -> Option<()> {
    let (first, last, n) = (d.get(w, 40, 8), d.get(w, 48, 8), d.get(w, 80, 4) as usize);
    let used: Vec<usize> = (0..n).filter(|&i| d.entry(w, i)[..16] != [0; 16]).collect();
    let free: Vec<usize> = (0..n).filter(|&i| d.entry(w, i)[..16] == [0; 16]).collect();
    let u = pick(r, &used)?;
    let (a, b) = span(d, w, u);
    match lie {
        Lie::Overlap => {
            let f = pick(r, &free)?;
            let span = match r.range(0, 2) {
                // Touching it at an edge sector: one sector shared, no more.
                0 => (r.range(first, a), a),
                1 => (b, r.range(b, last)),
                _ => (r.range(first, a), r.range(b, last)),
            };
            let kind: Vec<u8> = r.bytes(16).iter().map(|x| x | 1).collect();
            d.entry(w, f)[..16].copy_from_slice(&kind);
            set(d, w, f, span.0, span.1);
        }
        Lie::PastLast => {
            let past = [last + r.range(1, 1 << 20), d.sectors + r.range(0, 9), u64::MAX];
            set(d, w, u, a, r.one_of(&past));
        }
        Lie::EndBeforeStart => set(d, w, u, b + r.range(1, 64), b),
        Lie::BelowFirst => set(d, w, u, r.range(0, first - 1), b),
        other => unreachable!("{other:?} is not an entry lie"),
    }
    Some(())
}
