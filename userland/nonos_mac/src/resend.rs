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

//! A single received frame is handed to net.core once and kept until net.core
//! asks for the next one, so a reply that lands after net.core stopped
//! waiting is not lost: net.core asks again with the same number.
//!
//! net.core numbers each OP_RX_PACKET with a 4-byte body and moves to the
//! next number only on a reply it read with a frame in it. A request with no
//! body is the old wire: a fresh frame, nothing kept. Batch drivers already
//! keep a batch by number; this is the same for drivers serving one frame.

/// The last frame handed out, and the number it answered.
pub struct Kept<const N: usize> {
    seq: Option<u32>,
    len: usize,
    frame: [u8; N],
}

impl<const N: usize> Kept<N> {
    pub const fn new() -> Self {
        Kept { seq: None, len: 0, frame: [0; N] }
    }
}

impl<const N: usize> Default for Kept<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// The number an OP_RX_PACKET request asks for: its 4-byte body, if any.
pub fn asked(payload: &[u8]) -> Option<u32> {
    Some(u32::from_le_bytes(payload.get(..4)?.try_into().ok()?))
}

/// The frame to answer `asked` with: the kept one again when it answered the
/// same number, else a fresh one from `fresh` (which fills a buffer and says
/// how much, `None` for nothing queued), kept under `asked`.
pub fn serve<const N: usize, E>(
    kept: &mut Kept<N>,
    asked: Option<u32>,
    fresh: impl FnOnce(&mut [u8]) -> Result<Option<usize>, E>,
) -> Result<Option<&[u8]>, E> {
    if asked.is_some() && kept.seq == asked {
        return Ok(Some(&kept.frame[..kept.len]));
    }
    kept.seq = None;
    let Some(len) = fresh(&mut kept.frame)?.filter(|&n| n <= N) else {
        return Ok(None);
    };
    kept.len = len;
    kept.seq = asked;
    Ok(Some(&kept.frame[..len]))
}
