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

//! The display connection's two queues are this capsule's memory. A client
//! that sends requests and never reads their answers is held as a socket
//! holds a writer whose reader has stopped: the server stops serving once
//! MOST_UNREAD waits for the client, the requests wait in turn, and a write
//! past MOST_UNSENT takes nothing. Neither queue grows past its bound plus
//! one answer, however long the client keeps at it.

use crate::linux::unix::conn::{Conn, MOST_UNREAD, MOST_UNSENT};
use crate::wire::walk;

/// wl_display.sync on new id 2: the request a hostile client repeats.
const SYNC: [u8; 12] = [1, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0];
/// What the server answers a sync with: wl_callback.done and delete_id.
const ANSWER: usize = 24;

/// The client's write as sock_io::send takes it, then serve::serve's walk,
/// with each sync answered by ANSWER bytes. The count taken, as `send`
/// returns it; zero is EAGAIN.
fn write_and_serve(c: &mut Conn, bytes: &[u8]) -> usize {
    let take = bytes.len().min(c.room());
    c.to_server.extend_from_slice(&bytes[..take]);
    let mut queue = core::mem::take(&mut c.to_server);
    let served = match c.backlogged() {
        true => 0,
        false => walk(&queue, |_| {
            c.to_client.extend_from_slice(&[0u8; ANSWER]);
            !c.backlogged()
        }),
    };
    queue.drain(..served);
    c.to_server = queue;
    take
}

#[test]
fn a_client_that_never_reads_is_held_at_both_bounds() {
    let mut c = Conn::new();
    let batch: Vec<u8> = SYNC.iter().copied().cycle().take(SYNC.len() * 4096).collect();
    let mut refused = 0;
    for _ in 0..10_000 {
        if write_and_serve(&mut c, &batch) == 0 {
            refused += 1;
        }
        assert!(c.to_client.len() < MOST_UNREAD + ANSWER, "{}", c.to_client.len());
        assert!(c.to_server.len() <= MOST_UNSENT, "{}", c.to_server.len());
    }
    assert!(c.backlogged(), "the server stopped");
    assert_eq!(c.room(), 0, "and the client's writes take nothing");
    assert!(refused > 9_000, "EAGAIN from then on: {refused}");
}

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
