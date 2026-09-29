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

//! XXH64, whose low 32 bits are a frame's Content_Checksum.

const P1: u64 = 0x9E37_79B1_85EB_CA87;
const P2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const P3: u64 = 0x1656_67B1_9E37_79F9;
const P4: u64 = 0x85EB_CA77_C2B2_AE63;
const P5: u64 = 0x27D4_EB2F_1656_67C5;

fn round(acc: u64, lane: u64) -> u64 {
    acc.wrapping_add(lane.wrapping_mul(P2)).rotate_left(31).wrapping_mul(P1)
}

fn merge(h: u64, v: u64) -> u64 {
    (h ^ round(0, v)).wrapping_mul(P1).wrapping_add(P4)
}

fn word(d: &[u8]) -> u64 {
    d.iter().take(8).enumerate().fold(0, |a, (i, &b)| a | (b as u64) << (8 * i))
}

pub fn xxh64(data: &[u8], seed: u64) -> u64 {
    let mut stripes = data.chunks_exact(32);
    let mut h = if data.len() >= 32 {
        let mut v = [
            seed.wrapping_add(P1).wrapping_add(P2),
            seed.wrapping_add(P2),
            seed,
            seed.wrapping_sub(P1),
        ];
        for s in stripes.by_ref() {
            for (i, lane) in v.iter_mut().enumerate() {
                *lane = round(*lane, word(&s[i * 8..]));
            }
        }
        let h = v[0].rotate_left(1).wrapping_add(v[1].rotate_left(7));
        let h = h.wrapping_add(v[2].rotate_left(12)).wrapping_add(v[3].rotate_left(18));
        v.iter().fold(h, |h, &l| merge(h, l))
    } else {
        seed.wrapping_add(P5)
    };
    h = h.wrapping_add(data.len() as u64);
    let rest = stripes.remainder();
    let mut eights = rest.chunks_exact(8);
    for e in eights.by_ref() {
        h = (h ^ round(0, word(e))).rotate_left(27).wrapping_mul(P1).wrapping_add(P4);
    }
    let mut fours = eights.remainder().chunks_exact(4);
    for f in fours.by_ref() {
        h = (h ^ word(f).wrapping_mul(P1)).rotate_left(23).wrapping_mul(P2).wrapping_add(P3);
    }
    for &b in fours.remainder() {
        h = (h ^ (b as u64).wrapping_mul(P5)).rotate_left(11).wrapping_mul(P1);
    }
    h ^= h >> 33;
    h = h.wrapping_mul(P2);
    h ^= h >> 29;
    h = h.wrapping_mul(P3);
    h ^ (h >> 32)
}
