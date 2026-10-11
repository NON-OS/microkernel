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

//! Reading lets the server go on, a partial message finishes, the bounds are exact.

use super::held::{write_and_serve, SYNC};
use crate::linux::unix::conn::{Conn, MOST_UNREAD, MOST_UNSENT};

#[test]
fn reading_lets_the_server_go_on() {
    let mut c = Conn::new();
    let batch: Vec<u8> = SYNC.iter().copied().cycle().take(SYNC.len() * 4096).collect();
    for _ in 0..1_000 {
        if write_and_serve(&mut c, &batch) == 0 {
            break;
        }
    }
    assert_eq!(c.room(), 0, "held");
    let waiting = c.to_server.len();
    assert!(waiting > 0);
    let read = c.drain(usize::MAX);
    assert!(read.len() >= MOST_UNREAD);
    write_and_serve(&mut c, &[]);
    assert!(c.to_server.len() < waiting, "requests that waited are served");
    assert!(c.room() > 0, "and the client may write again");
}

/// A request is at most 65535 bytes, so one cut short is always taken
/// whole once it arrives: the bound never strands half a message.
#[test]
fn a_partial_message_always_has_room_to_finish() {
    let mut c = Conn::new();
    let mut head = vec![1u8, 0, 0, 0, 0, 0, 0xFF, 0xFF];
    head.resize(60_000, 0);
    assert_eq!(write_and_serve(&mut c, &head), head.len());
    assert!(c.room() >= 65_535 - head.len(), "the rest of it fits");
    assert_eq!(write_and_serve(&mut c, &[0u8; 5535]), 5535);
    assert!(c.to_server.is_empty(), "served whole");
}

#[test]
fn the_bounds_are_reached_exactly() {
    let mut c = Conn::new();
    c.to_client.resize(MOST_UNREAD - 1, 0);
    assert!(!c.backlogged());
    c.to_client.push(0);
    assert!(c.backlogged());
    c.to_server.resize(MOST_UNSENT - 1, 0);
    assert_eq!(c.room(), 1);
    c.to_server.push(0);
    assert_eq!(c.room(), 0);
    c.to_server.push(0);
    assert_eq!(c.room(), 0, "past the bound is still none");
}
