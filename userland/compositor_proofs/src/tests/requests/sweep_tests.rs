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

//! Every decoder against hostile bodies: each takes exactly the bodies
//! its op names and reads them as the wire defines, and each refusal
//! (short, long, or a word out of range) is met thousands of times.

use super::hostile::{body, edges, words};
use super::ops::{settings, Op};
use super::ops_other::others;
use super::ops_scene::pointer_and_scene;
use super::rng::Rng;
use core::cmp::Ordering;

pub fn all() -> Vec<Op> {
    [settings(), pointer_and_scene(), others()].concat()
}

#[test]
fn every_decoder_takes_exactly_the_bodies_its_op_names() {
    for (i, (name, len, good, decode, model)) in all().into_iter().enumerate() {
        let mut r = Rng::new(0x4E43_4D50_0000 + i as u64);
        let mut seen = [0usize; 4];
        for _ in 0..80_000 {
            let x = body(&mut r, len, good);
            let want = (x.len() == len).then(|| model(&words(&x))).flatten();
            assert_eq!(decode(&x), want, "{name} {x:02X?}");
            let k = match (want.is_some(), x.len().cmp(&len)) {
                (true, _) => 0,
                (false, Ordering::Less) => 1,
                (false, Ordering::Greater) => 2,
                (false, Ordering::Equal) => 3,
            };
            seen[k] += 1;
        }
        // [taken, short, long, refused for a value]; an op taking only 0
        // or 1 takes the fewest.
        assert!(seen[0] >= 1_500 && seen[1..3].iter().all(|&n| n >= 8_000), "{name} {seen:?}");
        let ranged = edges().any(|e| model(&[e; 8]).is_none());
        assert!(if ranged { seen[3] >= 2_000 } else { seen[3] == 0 }, "{name} {seen:?}");
    }
}

#[test]
fn every_op_in_the_sweep_is_a_distinct_decoder() {
    let ops = all();
    let mut names: Vec<_> = ops.iter().map(|o| o.0).collect();
    assert!(ops.iter().all(|o| (o.4)(o.2).is_some()), "every good body is taken");
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), ops.len());
    assert_eq!(ops.len(), 17);
}
