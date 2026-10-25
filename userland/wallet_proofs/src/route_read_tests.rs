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

//! The routed reader reads a late reply whole, stops at a whole flight, ends at a pause.

use super::route_script::{Script, BOUNDS};
use crate::wallet::net::routed_read::read_routed;

#[test]
fn a_reply_that_comes_late_and_in_pieces_is_read_whole() {
    let mut s = Script::new(vec![
        Some((20_000, b"HTTP/1.1 ".to_vec())),
        Some((3_000, b"200 OK\r\n".to_vec())),
        Some((7_000, b"\r\n{}".to_vec())),
        None,
    ]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), Some(b"HTTP/1.1 200 OK\r\n\r\n{}".to_vec()));
    /* The first wait is the route's patience, the rest the quiet window. */
    assert_eq!(s.waits[0], 60_000);
    assert!(s.waits[1..].iter().all(|&w| w <= 8_000), "{:?}", s.waits);
}

#[test]
fn a_flight_stops_as_soon_as_it_is_whole() {
    let mut s = Script::new(vec![
        Some((1_000, b"ab".to_vec())),
        Some((1_000, b"cd".to_vec())),
        Some((1_000, b"ef".to_vec())),
    ]);
    let got = read_routed(&mut s, BOUNDS, |b| b.len() >= 4);
    assert_eq!(got, Some(b"abcd".to_vec()));
    assert_eq!(s.steps.len(), 1, "nothing read past the end of the flight");
}

#[test]
fn a_pause_longer_than_the_quiet_window_ends_the_answer() {
    let mut s = Script::new(vec![Some((1_000, b"ab".to_vec())), Some((9_000, b"cd".to_vec()))]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), Some(b"ab".to_vec()));
}
