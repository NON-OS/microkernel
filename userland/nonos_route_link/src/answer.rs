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

/*
 * An answer from a proxy: one marker, then stream bytes, of which there may
 * be none. Nothing in it is believed before its length and its marker have
 * been checked, since it arrived from another process.
 */

/* The far end is open; what follows is stream bytes. */
pub const OPEN: u8 = 0;

/* The far end finished; what follows is the last of the stream. */
pub const CLOSED: u8 = 1;

/*
 * The proxy holds no conversation for this stream any more: it was
 * restarted under it, or ended and forgot it (capsule_socks5 server/run.rs,
 * capsule_net_anon server/socks/front.rs). The marker alone.
 */
pub const LOST: u8 = 2;

/*
 * The longest answer either proxy gives: a marker and 32 KiB of stream
 * bytes (net.socks5's ANSWER_MAX, net.anon's OUT_MAX and its marker).
 */
pub const ANSWER_MAX: usize = 1 + 32 * 1024;

/*
 * The buffer an answer is read into. The kernel cuts a reply to the buffer
 * without a word, so it is larger than any answer, and an answer that
 * reaches past ANSWER_MAX is refused rather than read as a stream with
 * bytes missing from its middle.
 */
pub const ANSWER_BUF: usize = 36 * 1024;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Answer<'a> {
    pub closed: bool,
    pub bytes: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Malformed {
    /* No marker: not an answer at all. */
    Empty,
    /* A marker neither proxy sends. */
    Marker(u8),
    /* Longer than any answer either proxy builds. */
    Oversized(usize),
    /* Not malformed: the proxy says it lost the conversation (`LOST`). */
    Lost,
}

/* The answer in `raw`, which holds exactly the bytes that arrived. */
pub fn decode(raw: &[u8]) -> Result<Answer<'_>, Malformed> {
    if raw.len() > ANSWER_MAX {
        return Err(Malformed::Oversized(raw.len()));
    }
    let (&marker, bytes) = raw.split_first().ok_or(Malformed::Empty)?;
    match marker {
        OPEN => Ok(Answer { closed: false, bytes }),
        CLOSED => Ok(Answer { closed: true, bytes }),
        LOST => Err(Malformed::Lost),
        other => Err(Malformed::Marker(other)),
    }
}

/*
 * The bytes that arrived, from a call that said `got` of them did into a
 * buffer of `room`: None when the count is negative or reaches past the
 * buffer, which no kernel answer can and no answer here is read beyond.
 */
pub fn arrived(got: i64, room: usize) -> Option<usize> {
    let n = usize::try_from(got).ok()?;
    (n <= room).then_some(n)
}
