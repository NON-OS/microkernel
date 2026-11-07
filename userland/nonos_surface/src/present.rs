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

//! The per-frame message a swapchain client sends the compositor: which of
//! its buffers its layer shows now, and what changed. No pixels: the
//! buffer is a surface the compositor already holds attached.

use super::swapchain_damage::Damage;

/// The compositor op (userland/compositor/src/protocol/ops.rs).
pub const OP_PRESENT: u16 = 0x001A;
pub const PRESENT_LEN: usize = 32;
/// The op naming the layer by the buffer it shows now, for a client with a
/// layer per window in one band: the present, then that handle.
pub const OP_PRESENT_LAYER: u16 = 0x0022;
pub const PRESENT_LAYER_LEN: usize = 40;
/// Set in the index word: every pixel of the buffer is opaque, so the
/// compositor may scan it out with nothing drawn under it.
pub const INDEX_OPAQUE: u32 = 1 << 31;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Present {
    /// The buffer's surface handle, as shared with the compositor.
    pub handle: u64,
    /// The layer's z, as it was submitted.
    pub z: u32,
    /// The buffer's index in its chain, for the trace.
    pub index: u32,
    pub damage: Damage,
    /// Every pixel is opaque (`INDEX_OPAQUE`).
    pub opaque: bool,
}

impl Present {
    pub fn encode(&self) -> [u8; PRESENT_LEN] {
        let mut b = [0u8; PRESENT_LEN];
        b[..8].copy_from_slice(&self.handle.to_le_bytes());
        let index = (self.index & !INDEX_OPAQUE) | if self.opaque { INDEX_OPAQUE } else { 0 };
        let words = [self.z, index, self.damage.x, self.damage.y];
        let more = [self.damage.width, self.damage.height];
        for (i, w) in words.iter().chain(more.iter()).enumerate() {
            b[8 + i * 4..12 + i * 4].copy_from_slice(&w.to_le_bytes());
        }
        b
    }
}

/// A PRESENT body made the OP_PRESENT_LAYER body for the layer showing `on`.
pub fn on_layer(body: &[u8; PRESENT_LEN], on: u64) -> [u8; PRESENT_LAYER_LEN] {
    let mut b = [0u8; PRESENT_LAYER_LEN];
    b[..PRESENT_LEN].copy_from_slice(body);
    b[PRESENT_LEN..].copy_from_slice(&on.to_le_bytes());
    b
}

/// The op naming its layer as OP_PRESENT_LAYER does, its changed box then
/// told apart as up to MAX_PARTS boxes inside it (a parts block), so two
/// rows changed far apart repaint those rows and not all between them.
pub const OP_PRESENT_LAYER_PARTS: u16 = 0x0024;
/// At most this many boxes in a parts block.
pub const MAX_PARTS: usize = 4;
/// A parts block: a count of 1 to MAX_PARTS, then MAX_PARTS boxes as x, y,
/// width and height, those past the count all zero.
pub const PARTS_LEN: usize = 4 + MAX_PARTS * 16;
pub const PRESENT_LAYER_PARTS_LEN: usize = PRESENT_LAYER_LEN + PARTS_LEN;

/// A box as x, y, width and height.
pub type Part = (u32, u32, u32, u32);

/// The parts block for `parts`; None for none or more than MAX_PARTS. That
/// each lies inside the request's own changed box is the caller's to hold;
/// the compositor refuses a block where one does not.
pub fn parts_block(parts: &[Part]) -> Option<[u8; PARTS_LEN]> {
    if parts.is_empty() || parts.len() > MAX_PARTS {
        return None;
    }
    let mut b = [0u8; PARTS_LEN];
    b[..4].copy_from_slice(&(parts.len() as u32).to_le_bytes());
    for (i, &(x, y, w, h)) in parts.iter().enumerate() {
        for (j, v) in [x, y, w, h].iter().enumerate() {
            let at = 4 + i * 16 + j * 4;
            b[at..at + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    Some(b)
}

/// An OP_PRESENT_LAYER body made the OP_PRESENT_LAYER_PARTS body with `parts`.
pub fn with_parts(
    body: &[u8; PRESENT_LAYER_LEN],
    parts: &[u8; PARTS_LEN],
) -> [u8; PRESENT_LAYER_PARTS_LEN] {
    let mut b = [0u8; PRESENT_LAYER_PARTS_LEN];
    b[..PRESENT_LAYER_LEN].copy_from_slice(body);
    b[PRESENT_LAYER_LEN..].copy_from_slice(parts);
    b
}
