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

//! The chunk scheduler (sched.rs): at most four streams, one chunk each in
//! flight, lowest first; a chunk whose stream broke goes to another; a
//! chunk is given up after its asks; a cache that holds everything asks
//! for nothing.

use crate::install::image::sched::{Sched, TRIES, WIDTH_MAX};

fn assigned(s: &mut Sched) -> Vec<(usize, usize)> {
    s.assign()
}

#[test]
fn width_is_capped_at_four_and_is_at_least_one() {
    assert_eq!(WIDTH_MAX, 4);
    assert_eq!(Sched::new(vec![false; 10], 9).width(), 4);
    assert_eq!(Sched::new(vec![false; 10], 0).width(), 1);
    let mut s = Sched::new(vec![false; 10], 99);
    assert_eq!(assigned(&mut s), [(0, 0), (1, 1), (2, 2), (3, 3)], "four at once, lowest first");
    assert!(assigned(&mut s).is_empty(), "none more while four are in flight");
}

#[test]
fn a_cache_that_holds_every_chunk_asks_for_none() {
    let mut s = Sched::new(vec![true; 24], 4);
    assert!(assigned(&mut s).is_empty());
    assert!(s.whole() && s.over());
}

#[test]
fn only_the_missing_chunks_are_asked_for() {
    let mut s = Sched::new(vec![true, false, true, false, false], 4);
    assert_eq!(assigned(&mut s), [(0, 1), (1, 3), (2, 4)]);
    for slot in 0..3 {
        s.done(slot, true);
    }
    assert!(s.whole() && s.over());
}

#[test]
fn a_broken_streams_chunk_is_asked_again_on_another() {
    let mut s = Sched::new(vec![false; 2], 2);
    assert_eq!(assigned(&mut s), [(0, 0), (1, 1)]);
    s.done(0, false);
    s.done(1, true);
    // Chunk 0 broke on stream 0; stream 1 is free and takes it.
    assert_eq!(assigned(&mut s), [(1, 0)]);
    s.done(1, true);
    assert!(s.whole());
}

#[test]
fn with_one_stream_a_broken_chunk_is_asked_again_on_it() {
    let mut s = Sched::new(vec![false], 1);
    assert_eq!(assigned(&mut s), [(0, 0)]);
    s.done(0, false);
    assert_eq!(assigned(&mut s), [(0, 0)]);
}

#[test]
fn a_chunk_is_given_up_after_its_asks_and_the_fetch_ends() {
    let mut s = Sched::new(vec![false; 3], 4);
    let mut asks = 0;
    while !s.over() {
        for (slot, chunk) in assigned(&mut s) {
            asks += 1;
            s.done(slot, chunk != 1);
        }
    }
    assert!(!s.whole(), "chunk 1 never came");
    assert_eq!(asks, 2 + TRIES as usize);
}

#[test]
fn no_chunk_is_ever_in_flight_twice() {
    let mut s = Sched::new(vec![false; 7], 4);
    let mut step = 0u32;
    while !s.over() {
        let a = assigned(&mut s);
        let flying: Vec<usize> = (0..4).filter_map(|slot| s.on(slot)).collect();
        let mut sorted = flying.clone();
        sorted.dedup();
        assert_eq!(sorted.len(), flying.len());
        for (slot, _) in a {
            step += 1;
            s.done(slot, !step.is_multiple_of(3));
        }
    }
    assert!(s.whole());
}
