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

use alloc::vec::Vec;

use super::read::{table, u16_at, u32_at};

/// The pair-adjustment subtables of a face's GPOS 'kern' feature, grouped
/// by lookup in lookup order, as absolute offsets into the font data. A
/// lookup of type 9 (extension) is followed to the pair subtable it wraps.
/// Empty when the face has no GPOS or no kern feature.
pub(super) fn kern_lookups(d: &[u8]) -> Vec<Vec<usize>> {
    let Some(gpos) = table(d, b"GPOS") else { return Vec::new() };
    let mut picked: Vec<u16> = kern_lookup_indices(d, gpos).unwrap_or_default();
    picked.sort_unstable();
    picked.dedup();
    let Some(list) = u16_at(d, gpos + 8).map(|o| gpos + o as usize) else { return Vec::new() };
    picked.iter().filter_map(|&i| pair_subtables(d, list, i)).filter(|s| !s.is_empty()).collect()
}

/* Lookup indices of every 'kern' feature in the FeatureList. */
fn kern_lookup_indices(d: &[u8], gpos: usize) -> Option<Vec<u16>> {
    let features = gpos + u16_at(d, gpos + 6)? as usize;
    let mut out = Vec::new();
    for f in 0..u16_at(d, features)? as usize {
        let rec = features + 2 + 6 * f;
        if d.get(rec..rec + 4) != Some(&b"kern"[..]) {
            continue;
        }
        let table = features + u16_at(d, rec + 4)? as usize;
        for k in 0..u16_at(d, table + 2)? as usize {
            out.push(u16_at(d, table + 4 + 2 * k)?);
        }
    }
    Some(out)
}

/* The pair subtables of lookup `index` in the LookupList at `list`. */
fn pair_subtables(d: &[u8], list: usize, index: u16) -> Option<Vec<usize>> {
    let lookup = list + u16_at(d, list + 2 + 2 * index as usize)? as usize;
    let kind = u16_at(d, lookup)?;
    let mut out = Vec::new();
    for s in 0..u16_at(d, lookup + 4)? as usize {
        let sub = lookup + u16_at(d, lookup + 6 + 2 * s)? as usize;
        match kind {
            2 => out.push(sub),
            9 if u16_at(d, sub + 2)? == 2 => out.push(sub + u32_at(d, sub + 4)? as usize),
            _ => {}
        }
    }
    Some(out)
}
