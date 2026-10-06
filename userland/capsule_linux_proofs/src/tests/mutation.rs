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

//! Damage the way hostile bytes arrive: a flipped bit, an overwritten byte,
//! a truncation, or a separator inserted where a parser splits.

pub fn damage(s: &mut u64, v: &mut Vec<u8>) {
    let mut next = || {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    };
    let at = (next() % v.len().max(1) as u64) as usize;
    match next() % 4 {
        0 if at < v.len() => v[at] ^= 1 << (next() % 8),
        1 if at < v.len() => v[at] = next() as u8,
        2 => v.truncate(at),
        _ => v.insert(at.min(v.len()), b"\n:%/ ."[(next() % 6) as usize]),
    }
}
