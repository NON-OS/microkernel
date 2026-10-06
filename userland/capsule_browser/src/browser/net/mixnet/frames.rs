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

//! The frames the browser sends a proxy, and how it reads the answers.
//!
//! Pure: no call is made here, so the proofs hold the very bytes the capsule
//! puts on the wire.

use alloc::vec::Vec;

/// The proxy marks every answer, so that an answer carrying no bytes is still
/// an answer. Without it "nothing yet" and "no reply at all" are the same
/// thing on the wire, and the caller waits out a timeout to tell them apart.
pub const STREAM_OPEN: u8 = 0;

/// The far end finished. Any bytes that follow are the last of the stream.
pub const STREAM_CLOSED: u8 = 1;

/// The proxy holds no conversation for the stream any more, and the
/// exchange asked was not the first of one: it was restarted under it, or
/// ended and forgot it (capsule_socks5 server/run.rs, capsule_net_anon
/// server/socks/front.rs). The marker alone. Told apart from a close, which
/// is the far end's, so the reader is not told the site hung up.
pub const STREAM_LOST: u8 = 2;

/// The longest answer either proxy builds: a marker and 32 KiB of stream
/// bytes (net.socks5's ANSWER_MAX, net.anon's OUT_MAX and its marker). The
/// kernel cuts a reply to the buffer without a word, so one longer than
/// this is refused rather than read with bytes missing from its middle.
pub const ANSWER_MAX: usize = 1 + 32 * 1024;

/// Tell the proxy to forget the previous conversation. It keys handshake
/// state on the caller, so without this a second page load meets a
/// connection still relaying the first, and its greeting is carried to the
/// exit as stream bytes rather than starting a handshake.
pub const STREAM_RESET: u8 = 1;

/// Stream bytes in a numbered exchange: marker, u32 number, bytes. A proxy
/// takes exit bytes out of its inbox to answer, and a call here waits only
/// the poll wait; the kernel drops a reply that arrives after that, and the
/// bytes in it were gone. Asked again with the same number and the same
/// bytes, the proxy gives back the answer it kept instead of carrying them a
/// second time (capsule_socks5 server/kept.rs, capsule_net_anon
/// server/socks/kept.rs), so they reach the exit once.
pub const STREAM_NUMBERED: u8 = 2;

/// A reset naming its stream: marker, u32 stream (never 0). A proxy keys a
/// conversation on our pid and the stream a frame names (capsule_socks5
/// server/who.rs), so the browser holds one conversation per stream rather
/// than one in all; frames naming none are stream 0.
pub const STREAM_RESET_ON: u8 = 3;

/// A numbered exchange naming its stream: marker, u32 stream, u32 number,
/// bytes.
pub const STREAM_NUMBERED_ON: u8 = 4;

/// The most stream bytes one frame carries. net.socks5 reads a request into
/// 34 KiB and net.anon into 32 KiB past its header; a TLS record of 16 KiB
/// and its overhead fit, with room. A longer write goes as several frames.
pub const CARRY_MAX: usize = 16 * 1024;

/// The first number of a conversation, after a reset.
pub const FIRST_SEQ: u32 = 1;

/// The frame that ends whatever conversation the proxy holds for us on
/// `stream`, 0 being the one a frame that names none belongs to.
pub fn reset(stream: u32) -> Vec<u8> {
    match stream {
        0 => Vec::from([STREAM_RESET]),
        s => [&[STREAM_RESET_ON][..], &s.to_le_bytes()].concat(),
    }
}

/// Exchange `seq` on `stream` carrying `bytes`, which may be none.
pub fn numbered(stream: u32, seq: u32, bytes: &[u8]) -> Vec<u8> {
    let mut framed = Vec::with_capacity(9 + bytes.len());
    match stream {
        0 => framed.push(STREAM_NUMBERED),
        s => {
            framed.push(STREAM_NUMBERED_ON);
            framed.extend_from_slice(&s.to_le_bytes());
        }
    }
    framed.extend_from_slice(&seq.to_le_bytes());
    framed.extend_from_slice(bytes);
    framed
}

/// The number after `seq`, never zero. It only has to differ from the one
/// before it, which a wrap still does.
pub fn next_seq(seq: u32) -> u32 {
    seq.wrapping_add(1).max(1)
}

/// An answer, read: whether the far end finished, and the stream bytes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Answer<'a> {
    pub closed: bool,
    pub bytes: &'a [u8],
}

/// The answer in `raw`, or `None` for bytes that are not one: no marker, a
/// marker neither proxy sends with stream bytes, or longer than any answer.
pub fn answer(raw: &[u8]) -> Option<Answer<'_>> {
    if raw.len() > ANSWER_MAX {
        return None;
    }
    match raw.split_first()? {
        (&STREAM_OPEN, bytes) => Some(Answer { closed: false, bytes }),
        (&STREAM_CLOSED, bytes) => Some(Answer { closed: true, bytes }),
        _ => None,
    }
}

/// Whether `raw` says the proxy lost the conversation (`STREAM_LOST`).
pub fn lost(raw: &[u8]) -> bool {
    raw == [STREAM_LOST]
}
