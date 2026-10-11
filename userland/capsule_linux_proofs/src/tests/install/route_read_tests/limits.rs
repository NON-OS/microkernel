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

//! Short replies, the size limit, the chunk asked for, and which network.

use super::script::{split, Script};
use crate::install::http_reply::{body, complete};
use crate::install::route_read::read::CHUNK;
use crate::install::route_read::{offline, read_reply, Reply};

#[test]
fn a_reply_that_stops_short_of_its_length_is_no_body() {
    let mut s = Script::new(split(&[30]));
    s.pieces.pop();
    let Reply::Got(raw) = read_reply(&mut s, 1 << 20, &complete) else { panic!("something came") };
    assert!(!complete(&raw));
    assert_eq!(body(raw), None);
}

#[test]
fn more_than_the_limit_is_refused_whole() {
    let big = vec![b'x'; 100];
    let mut s = Script::new(vec![Ok(big.clone()), Ok(big)]);
    assert_eq!(read_reply(&mut s, 150, &|_| false), Reply::TooLarge);
}

#[test]
fn a_read_asks_for_a_bounded_chunk() {
    let mut s = Script::new(vec![Ok(vec![0u8; CHUNK])]);
    s.ends_after_last = true;
    assert_eq!(read_reply(&mut s, usize::MAX, &|_| false), Reply::Got(vec![0u8; CHUNK]));
}

#[test]
fn an_install_waits_for_no_network_to_reach_a_local_mirror() {
    assert_eq!(offline(true, || None), None);
    assert_eq!(offline(true, || Some("Nym is not running")), None);
    /* The network is not even asked: asking waits up to three minutes. */
    assert_eq!(offline(true, || panic!("a local mirror waited for the network")), None);
}

#[test]
fn any_other_mirror_needs_the_chosen_network_running() {
    assert_eq!(offline(false, || None), None);
    assert_eq!(offline(false, || Some("Nym is not running")), Some("Nym is not running"));
}
