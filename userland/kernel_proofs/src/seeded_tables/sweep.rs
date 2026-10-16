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

//! One sweep: a table made from each seed, parsed, and what loaded held
//! to its bounds. Both answers must turn up often, so the damage is seen
//! to reach the parser and the parser is seen to take sound tables.

use super::gen::{structure, Rng};

/// Seeds each parser meets.
pub const SEEDS: u64 = 3000;

pub fn sweep<T>(
    make: impl Fn(&mut Rng) -> Vec<u8>,
    parse: impl Fn(&[u8]) -> Option<T>,
    check: impl Fn(&T, &[u8]) -> bool,
) {
    let (mut ok, mut refused) = (0, 0);
    for seed in 0..SEEDS {
        let t = make(&mut Rng::new(seed));
        match parse(&t) {
            Some(v) => {
                ok += 1;
                assert!(check(&v, &t), "seed {seed} loaded past its bounds");
            }
            None => refused += 1,
        }
    }
    let floor = SEEDS / 10;
    assert!(ok > floor && refused > floor, "both answers reached: {ok} {refused}");
}

/// `fixed` random bytes, then up to a dozen structures, each of a listed
/// kind and size or, one time in six, of a random kind and size.
pub fn body(r: &mut Rng, fixed: usize, kinds: &[(u8, usize)]) -> Vec<u8> {
    let mut b = r.bytes(fixed);
    for _ in 0..r.below(12) {
        let (ty, len) = if r.one_in(6) { (r.next() as u8, r.below(40)) } else { r.pick(kinds) };
        b.extend(structure(r, ty, len));
    }
    b
}

/// The bytes the declared length and the bytes handed over both cover.
pub fn held(t: &[u8]) -> usize {
    t.get(4..8).map_or(0, |h| u32::from_le_bytes(h.try_into().unwrap()) as usize).min(t.len())
}

/// The parser's answer, held to the rule every checked header keeps: a
/// table loads only when its declared length fits the bytes handed over
/// and those bytes sum to zero.
pub fn headed<T, E>(parse: impl Fn(&[u8]) -> Result<T, E>) -> impl Fn(&[u8]) -> Option<T> {
    move |t| {
        let v = parse(t).ok()?;
        let n = u32::from_le_bytes(t[4..8].try_into().unwrap()) as usize;
        assert!(n <= t.len(), "loaded with a length past the bytes");
        assert_eq!(t[..n].iter().fold(0u8, |s, b| s.wrapping_add(*b)), 0, "loaded unsummed");
        Some(v)
    }
}
