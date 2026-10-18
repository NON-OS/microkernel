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

//! A client that never reads is held at both bounds.

use crate::linux::unix::conn::{Conn, MOST_UNREAD, MOST_UNSENT};
use crate::wire::walk;

/// wl_display.sync on new id 2: the request a hostile client repeats.
pub(super) const SYNC: [u8; 12] = [1, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0];

/// What the server answers a sync with: wl_callback.done and delete_id.
const ANSWER: usize = 24;

/// The client's write as sock_io::send takes it, then serve::serve's walk,
/// with each sync answered by ANSWER bytes. The count taken, as `send`
/// returns it; zero is EAGAIN.
pub(super) fn write_and_serve(c: &mut Conn, bytes: &[u8]) -> usize {
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
