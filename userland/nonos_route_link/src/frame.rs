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
 * The frames sent to net.socks5 and net.anon, which both speak SOCKS5 as
 * bytes over IPC on their service ports. Every frame opens with a marker so
 * that one carrying no bytes (has anything come?) can be sent at all; the
 * kernel refuses an empty message.
 *
 * Stream bytes go numbered: the marker, a u32 little endian, the bytes. The
 * proxy takes bytes off the stream to build an answer, and the kernel drops
 * an answer whose caller stopped waiting, so those bytes were lost. A caller
 * that asks again under the same number, with the same bytes, is given the
 * same answer instead (capsule_socks5 server/kept.rs, capsule_net_anon
 * server/socks/kept.rs). The number moves on only once an answer arrives.
 */

use alloc::vec::Vec;

/* Forget this caller's conversation, ending any tunnel it held. */
pub const RESET: u8 = 1;

/* Stream bytes in a numbered exchange. */
pub const NUMBERED: u8 = 2;

/* The marker and the number in front of the bytes. */
pub const HEAD: usize = 5;

/*
 * The most stream bytes one frame carries. net.socks5 reads a request into
 * 34 KiB and net.anon into 32 KiB past its header; a TLS record of 16 KiB
 * and its overhead fit, with room.
 */
pub const CARRY_MAX: usize = 16 * 1024;

/* The first number of a conversation, after a reset. */
pub const FIRST_SEQ: u32 = 1;

pub fn reset() -> [u8; 1] {
    [RESET]
}

/* Exchange `seq` carrying `bytes`, or None when they do not fit one frame. */
pub fn numbered(seq: u32, bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() > CARRY_MAX {
        return None;
    }
    let mut out = Vec::with_capacity(HEAD + bytes.len());
    out.push(NUMBERED);
    out.extend_from_slice(&seq.to_le_bytes());
    out.extend_from_slice(bytes);
    Some(out)
}

/*
 * The number after `seq`, never zero, as the browser counts. The number
 * only has to differ from the one before it, which a wrap still does.
 */
pub fn next_seq(seq: u32) -> u32 {
    seq.wrapping_add(1).max(1)
}
