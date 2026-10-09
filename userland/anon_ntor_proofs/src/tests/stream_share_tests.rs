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

//! One caller's share of net.anon's streams, and the streams of callers that
//! ended without closing them.

use crate::stream::{may_open, orphaned, Stream, StreamStage, FRONT};

/// net.anon's STREAM_MAX in protocol/limits.rs.
const MAX: usize = 32;

fn owned_by(id: u16, owner: u32) -> Stream {
    let mut s = Stream::new(id, 1);
    s.owner = owner;
    s
}

#[test]
fn a_caller_stops_at_half_and_another_still_opens() {
    let streams: Vec<Stream> = (1..=16).map(|id| owned_by(id, 70)).collect();
    assert!(!may_open(&streams, 70, MAX), "past its half");
    assert!(may_open(&streams, 71, MAX), "another caller still opens");
}

#[test]
fn a_full_table_refuses_everyone() {
    let streams: Vec<Stream> = (1..=32).map(|id| owned_by(id, 100 + u32::from(id % 3))).collect();
    for owner in [100, 101, 102, 103] {
        assert!(!may_open(&streams, owner, MAX));
    }
}

#[test]
fn an_ended_caller_s_streams_are_orphans_ended_or_not() {
    let mut streams = vec![owned_by(1, 80), owned_by(2, 81), owned_by(3, 80)];
    streams[2].stage = StreamStage::Ended(6);
    assert_eq!(orphaned(&streams, |pid| pid != 80), vec![1, 3]);
}

#[test]
fn living_callers_and_the_socks_front_keep_their_streams() {
    let streams = vec![owned_by(1, FRONT), owned_by(2, 90)];
    assert!(orphaned(&streams, |pid| pid == 90).is_empty(), "the front's streams are its own");
    assert_eq!(orphaned(&streams, |_| false), vec![2]);
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn a_table_grown_only_through_may_open_keeps_both_bounds() {
    let mut s = 0xA11C_E5ED_u64;
    let mut streams: Vec<Stream> = Vec::new();
    let mut id = 0u16;
    for _ in 0..20_000 {
        let r = xorshift(&mut s);
        let owner = 1 + (r % 5) as u32;
        if r & 0x100 != 0 && !streams.is_empty() {
            let at = (r >> 16) as usize % streams.len();
            streams.swap_remove(at);
        } else if may_open(&streams, owner, MAX) {
            id = id.wrapping_add(1).max(1);
            streams.push(owned_by(id, owner));
        }
        assert!(streams.len() <= MAX);
        for o in 1..=5 {
            assert!(streams.iter().filter(|x| x.owner == o).count() <= MAX / 2);
        }
    }
}
