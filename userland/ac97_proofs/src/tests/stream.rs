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

//! A stream from start: the engine plays the four silent periods it
//! started on, then exactly the bytes the server wrote, in order, for as
//! long as the driver keeps looking.

use super::feed::{pattern, period};
use super::util::{ask, request, status, up};
use crate::constants::ring::{PERIOD_BYTES, RING_BYTES};
use crate::server::{refill, Stream};

#[test]
fn a_stream_plays_its_silence_then_what_was_written() {
    let (mut d, ich) = up();
    let mut st = Stream::default();
    assert_eq!(status(&ask(&mut d, &mut st, &request(8, 2, &[]))), 0);
    {
        let ich = ich.borrow();
        assert_eq!((ich.bdbar, ich.lvi, ich.cr & 1), (d.bdl_dev(), 31, 1));
        assert!(ich.open(), "outputs open while the stream runs");
    }
    let total = 24 * PERIOD_BYTES;
    let mut fed = 0;
    for _ in 0..30 {
        period(&mut d, &mut st, &ich, &mut fed, total);
    }
    let ich = ich.borrow();
    assert!(ich.heard[..RING_BYTES].iter().all(|&b| b == 0), "the ring starts silent");
    let played = &ich.heard[RING_BYTES..RING_BYTES + total];
    assert!(played.iter().enumerate().all(|(i, &b)| b == pattern(i)), "every byte, in order");
    assert_eq!(ich.halts, 0, "the engine never reached the last valid entry");
}

// Over a hundred periods the index wraps three times; the last valid one
// stays just behind it.
#[test]
fn the_last_valid_index_trails_the_engine_round_the_list() {
    let (mut d, ich) = up();
    let mut st = Stream::default();
    assert!(st.start(&mut d));
    for _ in 0..100 {
        ich.borrow_mut().play(PERIOD_BYTES);
        refill(&mut d, &mut st);
        let ich = ich.borrow();
        assert_eq!(ich.lvi, (ich.civ + 31) % 32);
        assert_eq!(ich.sr & 1, 0, "still running");
    }
}
