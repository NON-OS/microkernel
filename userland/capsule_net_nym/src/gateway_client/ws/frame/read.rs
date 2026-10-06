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

// These share a space with what net.tcp reports through the same call, and
// with the timeout and close below. They used to be 8 and 9, the same as a
// timeout and a close and as a net.tcp call that never completed: a frame
// that did not fit read as a quiet link, and the bytes held with it were
// dropped without a word.

/// A frame longer than the buffer it was to be copied into.
pub const E_NEED_MORE: u16 = 202;
/// Bytes that are not a frame: the link has lost its place in the stream.
pub const E_BAD_FRAME: u16 = 203;

/// The longest frame a buffer has to take. The length field reaches 65535
/// before it needs the 64 bit form, which this refuses, so a buffer this
/// size holds any frame that will be accepted. A gateway pushes a reply as
/// large as the packet it came in, and a requester may answer in packets of
/// up to 32 KiB; a buffer sized for the 2 KiB regular packet turned every
/// larger one into an error that read as an idle link.
pub const FRAME_MAX: usize = 64 * 1024;

pub fn frame_len(buf: &[u8], len: usize) -> Result<Option<(usize, usize)>, u16> {
    if len == 126 {
        if buf.len() < 4 {
            return Ok(None);
        }
        return Ok(Some((u16::from_be_bytes([buf[2], buf[3]]) as usize, 4)));
    }
    if len == 127 {
        return Err(E_BAD_FRAME);
    }
    Ok(Some((len, 2)))
}

pub fn copy_payload(
    buf: &[u8],
    out: &mut [u8],
    masked: bool,
    len: usize,
    off: usize,
) -> Result<usize, u16> {
    if len > out.len() {
        return Err(E_NEED_MORE);
    }
    let mask = if masked { &buf[off..off + 4] } else { &[0, 0, 0, 0] };
    let start = off + if masked { 4 } else { 0 };
    for i in 0..len {
        out[i] = buf[start + i] ^ mask[i % 4];
    }
    Ok(len)
}
