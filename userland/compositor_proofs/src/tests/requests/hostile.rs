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

//! Bodies a hostile client might send: the right length or a lie about
//! it, cut short or run long, each word an edge of some bound or noise.

use super::rng::Rng;

/// Values on either side of every bound a decoder keeps, so each bound
/// is met from both sides far more often than noise would.
const LOW: [u32; 22] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 16, 24, 32, 33, 0xFF, 0x100, 999, 1000, 1001, 0xFFFF,
    0x1_0000,
];
const HIGH: [u32; 4] = [u32::MAX >> 1, 1 << 31, u32::MAX - 1, u32::MAX];

/// Every edge.
pub fn edges() -> impl Iterator<Item = u32> {
    LOW.into_iter().chain(HIGH)
}

/// A word: an edge most of the time, else any value.
pub fn word(r: &mut Rng) -> u32 {
    let i = r.range(0, (LOW.len() + HIGH.len()) as u32 - 1) as usize;
    if r.one_in(3) {
        r.word()
    } else {
        edges().nth(i).unwrap_or(0)
    }
}

/// A body for an op that takes `len` bytes: exactly that most of the
/// time, else a few bytes either side, else any length to 64. Half start
/// from `good`, a body the op takes, with up to two words changed.
pub fn body(r: &mut Rng, len: usize, good: &[u32]) -> Vec<u8> {
    let n = match r.range(0, 5) {
        0 => r.range(0, 64) as usize,
        1 => len.saturating_sub(r.range(1, 4) as usize),
        2 => len + r.range(1, 4) as usize,
        _ => len,
    };
    let mut w: Vec<u32> = (0..n.div_ceil(4)).map(|_| word(r)).collect();
    if r.one_in(2) {
        w.iter_mut().zip(good).for_each(|(v, g)| *v = *g);
        for _ in 0..r.range(0, 2) {
            let i = r.range(0, w.len().max(1) as u32 - 1) as usize;
            if let Some(v) = w.get_mut(i) {
                *v = word(r);
            }
        }
    }
    let mut b: Vec<u8> = w.iter().flat_map(|v| v.to_le_bytes()).collect();
    b.truncate(n);
    b
}

/// The body's whole words, read the way the wire defines them.
pub fn words(b: &[u8]) -> Vec<u32> {
    b.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
