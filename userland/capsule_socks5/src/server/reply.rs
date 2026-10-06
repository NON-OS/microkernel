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

/// The tunnel is open and whatever follows is stream bytes, of which there
/// may be none.
pub const STREAM_OPEN: u8 = 0;

/// The far end finished. Any bytes that follow are the last of the stream.
pub const STREAM_CLOSED: u8 = 1;

/// This proxy holds no conversation for that stream, and the exchange asked
/// is not the first of one: it was lost when the proxy restarted, or ended
/// and forgotten. Said apart from a close, which is the far end's, so the
/// caller can tell the reader the proxy lost it rather than blaming the site.
/// The marker alone, no bytes follow.
pub const STREAM_LOST: u8 = 2;

/// The answer to a status ask (`request::STATUS_ASK`): this marker, a format
/// version, then whether a stream can be opened now, the step the network is
/// at and how many steps there are. Five bytes in all.
pub const STATUS: u8 = 3;

/// The format of a status answer.
pub const STATUS_VERSION: u8 = 1;

/// The steps net.socks5 reports: waiting for net.nym to start, opening its
/// mixnet session, and trying another exit after one did not answer. Ready
/// once a session is open on an exit not just taken after a silent one.
pub const STEP_WAITING_FOR_NYM: u8 = 1;
pub const STEP_OPENING_SESSION: u8 = 2;
pub const STEP_TRYING_ANOTHER_EXIT: u8 = 3;
pub const STEPS: u8 = 3;

/// The answer to a status ask, with how many exits this session walked away
/// from for silence after the five bytes both proxies give. A reader that
/// knows only the five takes the sixth as no answer, as an older one would.
pub fn progress(ready: bool, step: u8, silent: u8) -> Vec<u8> {
    Vec::from([STATUS, STATUS_VERSION, u8::from(ready), step, STEPS, silent])
}

/// The most stream bytes one answer carries, marker aside. Every caller reads
/// an answer into 36 KiB, and the kernel cuts a reply longer than the buffer
/// it is read into without a word, which takes bytes out of the middle of the
/// stream; what does not fit waits in the inbox for the next answer.
pub const ANSWER_MAX: usize = 32 * 1024;

/// What the proxy says back to one request.
///
/// A caller waiting on a mixnet reply asks repeatedly and is usually told
/// there is nothing yet, so "nothing" has to be sayable. The kernel refuses a
/// zero length reply, which left silence as the only way to express it, and
/// silence is not an answer: the caller blocks until its own timeout expires
/// on an answer already known. One leading byte makes the empty answer a real
/// one, and carries whether the tunnel is still open while it is there.
pub struct Reply {
    pub bytes: Vec<u8>,
    pub closed: bool,
}

impl Reply {
    pub fn open(bytes: Vec<u8>) -> Self {
        Self { bytes, closed: false }
    }

    pub fn closed(bytes: Vec<u8>) -> Self {
        Self { bytes, closed: true }
    }

    /// The bytes to put on the wire, marker first.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(1 + self.bytes.len());
        out.push(if self.closed { STREAM_CLOSED } else { STREAM_OPEN });
        out.extend_from_slice(&self.bytes);
        out
    }
}
