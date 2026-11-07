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

//! A seeded sweep over the parsers on bytes nobody vouched for: every single
//! byte of a valid head and list changed, and random truncations and
//! splices. Neither parser panics, and whatever it accepts lies inside its
//! input. The libFuzzer target (fuzz/) runs the same checks unbounded.

use super::common;

use common::{image, list};
use nonos_app_image::{covering, ChunkList, Head, CHUNK_MIN};

fn accepted_head_is_inside(b: &[u8]) {
    if let Ok(h) = Head::parse(b) {
        assert!(!h.members.is_empty());
        let mut at = h.members[0].at.start;
        for m in &h.members {
            assert_eq!(m.at.start, at, "members tile");
            assert!(m.at.end <= h.total);
            at = m.at.end;
        }
        assert_eq!(at, h.total);
    }
}

fn accepted_list_is_inside(b: &[u8]) {
    if let Ok(c) = ChunkList::read(b) {
        for i in 0..c.count() {
            let r = c.range(i).expect("a counted chunk has a range");
            assert!(r.start < r.end && r.end <= c.total);
        }
        assert!(covering(&c, &(0..c.total)).len() == c.count());
    }
}

#[test]
fn every_byte_of_a_head_changed() {
    let img = image();
    let n = nonos_app_image::head_len(&img).expect("head");
    for at in 0..n {
        for v in [0u8, 1, 0x7f, 0x80, 0xff] {
            let mut b = img.clone();
            b[at] = v;
            accepted_head_is_inside(&b);
            accepted_head_is_inside(&b[..n]);
        }
    }
}

#[test]
fn every_byte_of_a_list_changed() {
    let (l, _) = list(&image(), CHUNK_MIN);
    for at in 0..l.len() {
        for v in [0u8, 1, 0x7f, 0x80, 0xff] {
            let mut b = l.clone();
            b[at] = v;
            accepted_list_is_inside(&b);
        }
    }
}

#[test]
fn seeded_truncations_and_noise() {
    let img = image();
    let (l, _) = list(&img, CHUNK_MIN);
    let mut s: u64 = std::env::var("NONOS_FUZZ_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(7);
    let iters: usize = std::env::var("NONOS_FUZZ_ITERS").ok().and_then(|v| v.parse().ok()).unwrap_or(20_000);
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    for _ in 0..iters {
        let (src, head) = if rnd() % 2 == 0 { (&img, true) } else { (&l, false) };
        let mut b = src[..(rnd() as usize % (src.len() + 1)).min(4096)].to_vec();
        for _ in 0..rnd() % 8 {
            if !b.is_empty() {
                let i = rnd() as usize % b.len();
                b[i] = rnd() as u8;
            }
        }
        match head {
            true => accepted_head_is_inside(&b),
            false => accepted_list_is_inside(&b),
        }
    }
}
