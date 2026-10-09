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

extern crate alloc;

use alloc::vec::Vec;

/// The most bytes held for one connection. A reader that lets this much pile
/// up has stopped reading, and holding more for it only takes memory from
/// the streams that are being read.
pub const CONN_HELD_MAX: usize = 1024 * 1024;

/// The most chunks held for one connection, so a stream of empty messages
/// cannot grow the table where the byte count would not notice.
pub const CONN_CHUNKS_MAX: usize = 1024;

/// The most bytes held across every connection.
pub const HELD_MAX: usize = 6 * 1024 * 1024;

/// How long bytes may wait behind a gap before the stream is given up on.
///
/// The exit resends a lost message once its acknowledgement fails to come
/// back, within seconds, and asks for reply blocks if it has run out. A gap
/// that has not filled in this long, with later bytes already here, is a
/// message that will not come: the stream is over, and saying so lets the
/// reader fail with a reason instead of waiting out its own patience.
pub const GAP_MS: i64 = 60_000;

/// What became of one chunk.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Accept {
    /// Held until the bytes in front of it have been read.
    Held,
    /// Its position was already read or is already held.
    Duplicate,
    /// Holding it would pass a bound, so the stream named here was ended
    /// instead: everything held for it was dropped and its reader is told
    /// the stream is over. When that is the chunk's own stream the chunk was
    /// not held.
    Overflow(u64),
}

/// A chunk that arrived before the bytes in front of it.
struct Held {
    conn: u64,
    seq: u64,
    closed: bool,
    data: Vec<u8>,
}

/// Where a connection's stream has been read up to.
struct Mark {
    conn: u64,
    /// The next message number expected.
    next: u64,
    /// When the stream last moved: its first chunk arriving, or a read
    /// taking the next one in order.
    moved_ms: i64,
}

/// Stream messages coming back, put back in order.
///
/// The mixnet delays every packet on purpose, so pieces of one stream arrive
/// in whatever order their delays happened to produce. Handing them on as
/// they land would corrupt the stream, and a page assembled out of order
/// fails in ways that look like anything but the network.
///
/// The far end numbers each message it sends, counting messages rather than
/// bytes, and expects them read back in that order.
#[derive(Default)]
pub struct Inbox {
    held: Vec<Held>,
    marks: Vec<Mark>,
    /// Bytes held across every connection.
    bytes: usize,
    /// The clock stalls are measured on, set by the caller before each use.
    now_ms: i64,
}

impl Inbox {
    /// Set the clock that stalls are measured against.
    pub fn tick(&mut self, now_ms: i64) {
        self.now_ms = now_ms;
    }

    /// Bytes held across every connection.
    pub fn held_bytes(&self) -> usize {
        self.bytes
    }

    /// Hold one chunk of `conn`'s stream until it can be read in order.
    ///
    /// A position already read, or already held, is a copy: the mixnet can
    /// deliver one twice, and holding it again kept it for ever, since the
    /// stream had moved past the only place it could be read.
    pub fn accept(&mut self, conn: u64, seq: u64, closed: bool, data: &[u8]) -> Accept {
        let next = self.mark(conn);
        if seq < next || self.held.iter().any(|h| h.conn == conn && h.seq == seq) {
            return Accept::Duplicate;
        }
        if !self.marks.iter().any(|m| m.conn == conn) {
            self.marks.push(Mark { conn, next: 0, moved_ms: self.now_ms });
        }
        let (chunks, bytes) = self.held_for(conn);
        if chunks >= CONN_CHUNKS_MAX || bytes.saturating_add(data.len()) > CONN_HELD_MAX {
            self.close_now(conn);
            return Accept::Overflow(conn);
        }
        let mut ended = None;
        if self.bytes.saturating_add(data.len()) > HELD_MAX {
            let victim = self.heaviest().unwrap_or(conn);
            self.close_now(victim);
            if victim == conn {
                return Accept::Overflow(conn);
            }
            ended = Some(victim);
        }
        self.bytes = self.bytes.saturating_add(data.len());
        self.held.push(Held { conn, seq, closed, data: data.to_vec() });
        match ended {
            Some(victim) => Accept::Overflow(victim),
            None => Accept::Held,
        }
    }

    /// Take what continues `conn`'s stream, in order, at most `room` bytes.
    /// Returns the bytes and whether the far end finished. A chunk too long
    /// for the room left is split, its tail held at the same position, so
    /// the close it may carry is reported with its last byte.
    pub fn drain(&mut self, conn: u64, room: usize) -> (Vec<u8>, bool) {
        let mut out = Vec::new();
        let mut done = false;
        loop {
            let want = self.mark(conn);
            let Some(i) = self.held.iter().position(|h| h.conn == conn && h.seq == want) else {
                break;
            };
            let left = room.saturating_sub(out.len());
            if self.held[i].data.len() > left {
                let tail = self.held[i].data.split_off(left);
                out.extend_from_slice(&self.held[i].data);
                self.held[i].data = tail;
                self.bytes = self.bytes.saturating_sub(left);
                break;
            }
            let chunk = self.held.remove(i);
            self.bytes = self.bytes.saturating_sub(chunk.data.len());
            out.extend_from_slice(&chunk.data);
            done |= chunk.closed;
            // Positions count messages, not bytes: the far end numbers each
            // one and expects them read back in that order. Advancing by the
            // length of what arrived asks for a position nothing will ever
            // carry, and the stream stops at the first chunk.
            self.set_mark(conn, want.wrapping_add(1));
        }
        (out, done)
    }

    /// The position `conn`'s stream is stuck on, if bytes behind a gap have
    /// waited longer than `GAP_MS` without the stream moving.
    ///
    /// Only a gap counts. A stream with nothing held is idle, which is a
    /// connection waiting for its next request, not one that has failed.
    pub fn stalled(&self, conn: u64) -> Option<u64> {
        let mark = self.marks.iter().find(|m| m.conn == conn)?;
        if !self.held.iter().any(|h| h.conn == conn && h.seq > mark.next) {
            return None;
        }
        (self.now_ms.saturating_sub(mark.moved_ms) > GAP_MS).then_some(mark.next)
    }

    /// Forget a connection, along with anything still held for it.
    pub fn forget(&mut self, conn: u64) {
        let mut freed = 0usize;
        self.held.retain(|h| {
            let keep = h.conn != conn;
            if !keep {
                freed = freed.saturating_add(h.data.len());
            }
            keep
        });
        self.bytes = self.bytes.saturating_sub(freed);
        self.marks.retain(|m| m.conn != conn);
    }

    /// End `conn` from this side, discarding whatever was still held.
    ///
    /// Used when the exit is rotated away: everything in flight was
    /// addressed to a node that never answered, so the client is told the
    /// stream is over and reconnects through the replacement instead of
    /// waiting out a reply that cannot come.
    pub fn close_now(&mut self, conn: u64) {
        self.forget(conn);
        self.held.push(Held { conn, seq: 0, closed: true, data: Vec::new() });
    }

    fn held_for(&self, conn: u64) -> (usize, usize) {
        self.held
            .iter()
            .filter(|h| h.conn == conn)
            .fold((0, 0), |(n, b), h| (n + 1, b.saturating_add(h.data.len())))
    }

    /// The connection holding the most bytes.
    fn heaviest(&self) -> Option<u64> {
        let mut best: Option<(u64, usize)> = None;
        for m in &self.marks {
            let (_, bytes) = self.held_for(m.conn);
            if best.is_none_or(|(_, b)| bytes > b) {
                best = Some((m.conn, bytes));
            }
        }
        best.map(|(conn, _)| conn)
    }

    fn mark(&self, conn: u64) -> u64 {
        self.marks.iter().find(|m| m.conn == conn).map(|m| m.next).unwrap_or(0)
    }

    fn set_mark(&mut self, conn: u64, at: u64) {
        let now = self.now_ms;
        match self.marks.iter_mut().find(|m| m.conn == conn) {
            Some(m) => {
                m.next = at;
                m.moved_ms = now;
            }
            None => self.marks.push(Mark { conn, next: at, moved_ms: now }),
        }
    }
}
