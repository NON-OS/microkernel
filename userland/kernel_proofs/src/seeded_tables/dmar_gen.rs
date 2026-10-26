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
//! DMAR's remapping structures for the sweep (VT-d 3.x section 8): a 16-bit
//! kind and length, each kind's fixed fields, then device scopes or a
//! namespace name. A structure's length or a scope's sometimes lies, and a
//! structure is sometimes cut; RMRR regions are mostly whole pages, so the
//! parser is seen to keep some.

use super::gen::Rng;

/// The fixed bytes after kind and length: DRHD, RMRR, ATSR, RHSA, ANDD, SATC.
const FIXED: [usize; 6] = [12, 20, 4, 16, 4, 4];

/// A device scope: type, length, two reserved, enumeration id, start bus,
/// then (device, function) hops.
fn scope(r: &mut Rng) -> Vec<u8> {
    let hops = r.below(6);
    let mut s = vec![1 + r.below(5) as u8, (6 + 2 * hops) as u8, 0, 0];
    s.extend(r.bytes(2 + 2 * hops));
    if r.one_in(8) {
        s[1] = r.next() as u8;
    }
    s
}

/// One remapping structure, one time in eight of a kind no DMAR defines.
pub fn structure(r: &mut Rng) -> Vec<u8> {
    let kind = if r.one_in(8) { r.next() as u16 } else { r.below(6) as u16 };
    let fixed = FIXED.get(kind as usize).copied().unwrap_or_else(|| r.below(40));
    let mut body = r.bytes(fixed);
    match kind {
        1 if !r.one_in(4) => {
            let base = (r.below(1 << 20) as u64) << 12;
            let limit = (base + ((r.below(32) as u64) << 12)) | 0xFFF;
            body[4..20].copy_from_slice(&[base.to_le_bytes(), limit.to_le_bytes()].concat());
        }
        4 => {
            let name: Vec<u8> = (0..r.below(48)).map(|_| b'!' + r.below(94) as u8).collect();
            body.extend(name.iter().chain(&[0][..r.below(2)]));
        }
        _ => {}
    }
    if matches!(kind, 0 | 1 | 2 | 5) {
        (0..r.below(8)).for_each(|_| body.extend(scope(r)));
    }
    let length = if r.one_in(10) { r.next() as u16 } else { (body.len() + 4) as u16 };
    let mut s = [kind.to_le_bytes(), length.to_le_bytes()].concat();
    s.extend(body);
    if r.one_in(12) {
        s.truncate(r.below(s.len()));
    }
    s
}

/// A DMAR body: host address width, flags, ten reserved, then structures.
pub fn dmar_body(r: &mut Rng) -> Vec<u8> {
    let mut b = [vec![r.below(64) as u8, r.next() as u8], vec![0; 10]].concat();
    (0..r.below(12)).for_each(|_| b.extend(structure(r)));
    b
}
