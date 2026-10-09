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

//! A stream answers only the caller that opened it. Ids are sequential, so a
//! second caller can always guess one; it must still be refused.

use crate::stream::{carries_others, owned, Stream, FRONT};

fn opened_by(id: u16, circuit: u32, owner: u32) -> Stream {
    let mut s = Stream::new(id, circuit);
    s.owner = owner;
    s
}

#[test]
fn a_caller_reaches_its_own_stream_and_no_other() {
    let streams = vec![opened_by(1, 9, 40), opened_by(2, 9, 41)];
    assert_eq!(owned(&streams, 40, 1), Some(0));
    assert_eq!(owned(&streams, 41, 2), Some(1));
    assert_eq!(owned(&streams, 41, 1), None, "pid 41 reached pid 40's stream");
    assert_eq!(owned(&streams, 40, 2), None, "pid 40 reached pid 41's stream");
    assert_eq!(owned(&streams, 40, 3), None);
}

#[test]
fn no_api_caller_reaches_a_stream_the_socks_front_opened() {
    let streams = vec![opened_by(5, 9, FRONT)];
    for pid in [1, 2, 40, u32::MAX] {
        assert_eq!(owned(&streams, pid, 5), None, "pid {pid} reached a SOCKS stream");
    }
}

#[test]
fn a_circuit_carrying_another_callers_stream_is_not_the_callers_to_close() {
    let streams = vec![opened_by(1, 9, 40), opened_by(2, 9, FRONT), opened_by(3, 8, 40)];
    assert!(carries_others(&streams, 40, 9), "pid 40 could end the front's stream");
    assert!(!carries_others(&streams, 40, 8));
    assert!(!carries_others(&streams, 40, 7), "an empty circuit carries no one's stream");
}
