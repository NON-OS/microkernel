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
//! OP_PRESENT_LAYER's body: the layer that shows `on` shows `handle` from
//! now on, and only `area` of it changed, in the buffer's own pixels
//! (compositor requests/present.rs). Pure, so compositor_proofs decodes it
//! with the compositor's own reader.

pub const OP_PRESENT_LAYER: u16 = 0x0022;
pub const PRESENT_LAYER_LEN: usize = 40;

/// Set in the index word: every pixel of the frame is opaque.
pub const INDEX_OPAQUE: u32 = 1 << 31;

pub fn present_layer_body(
    (handle, on): (u64, u64),
    z: u32,
    (x, y, width, height): (u32, u32, u32, u32),
    opaque: bool,
) -> [u8; PRESENT_LAYER_LEN] {
    let mut b = [0u8; PRESENT_LAYER_LEN];
    b[0..8].copy_from_slice(&handle.to_le_bytes());
    // z, then the buffer's index in a chain, which the shell has none of,
    // and whether the frame is opaque.
    let index = if opaque { INDEX_OPAQUE } else { 0 };
    let words = [z, index, x, y, width, height];
    for (i, w) in words.iter().enumerate() {
        b[8 + i * 4..12 + i * 4].copy_from_slice(&w.to_le_bytes());
    }
    b[32..40].copy_from_slice(&on.to_le_bytes());
    b
}
