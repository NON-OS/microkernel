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

//! What driver.xhci0 sends back, checked before any of it is used. Every
//! length in a reply is the controller's word, so each is held to the bytes
//! that came, and no byte past them is read. Pure, so the host proofs feed
//! it hostile replies.

use super::wire::{DATA_AT, HDR_LEN, MAGIC};
use crate::protocol::E_IO;

fn word(b: &[u8], at: usize) -> Option<u32> {
    let w = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

fn half(b: &[u8], at: usize) -> Result<usize, i32> {
    let h = b.get(at..at + 2).ok_or(E_IO)?;
    Ok(u16::from_le_bytes([h[0], h[1]]) as usize)
}

/// The data length after the status word of the reply to request `rid`,
/// which put `n` bytes in `resp`; the controller's status when it is not 0;
/// `E_IO` when it is not a whole reply to that request. A payload length
/// past the bytes that came was taken, and the bytes never sent were read
/// as data.
pub fn reply(resp: &[u8], n: i64, rid: u32) -> Result<usize, i32> {
    let got = resp.get(..usize::try_from(n).map_err(|_| E_IO)?).ok_or(E_IO)?;
    let (Some(magic), Some(id), Some(plen), Some(status)) =
        (word(got, 0), word(got, 12), word(got, 16), word(got, HDR_LEN))
    else {
        return Err(E_IO);
    };
    let plen = plen as usize;
    if magic != MAGIC || id != rid || plen < 4 || HDR_LEN.saturating_add(plen) > got.len() {
        return Err(E_IO);
    }
    match status as i32 {
        0 => Ok(plen - 4),
        status => Err(status),
    }
}

/// The bytes a bulk transfer moved, from its reply of `len` data bytes.
pub fn moved(resp: &[u8], len: usize) -> Result<usize, i32> {
    if len < 4 {
        return Err(E_IO);
    }
    half(resp, DATA_AT)
}

/// The data a reply of `len` data bytes carries after a `head`-byte prefix
/// that opens with its count, copied to `out`. Returns the count.
pub fn data_in(resp: &[u8], len: usize, head: usize, out: &mut [u8]) -> Result<usize, i32> {
    let count = half(resp, DATA_AT)?;
    if count > out.len() || len < head.saturating_add(count) {
        return Err(E_IO);
    }
    let from = DATA_AT + head;
    out[..count].copy_from_slice(resp.get(from..from + count).ok_or(E_IO)?);
    Ok(count)
}
