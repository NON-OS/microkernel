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

//! OP_LAYER_CONTENT: the caller's layer showing `on` draws a box of another
//! surface over its own, from a corner inside it, cut to the layer; handle 0
//! draws none again. The surface it replaces is released after the next
//! frame.

use crate::swap::{parts_block, Part, PARTS_LEN};
use crate::wire::{call_status, NCMP_MAGIC};

pub const OP_LAYER_CONTENT: u16 = 0x0023;
pub const LAYER_CONTENT_LEN: usize = 64;
/// OP_LAYER_CONTENT, the box that changed told apart as up to MAX_PARTS
/// boxes inside it, each repainted alone.
pub const OP_LAYER_CONTENT_PARTS: u16 = 0x0025;
pub const LAYER_CONTENT_PARTS_LEN: usize = LAYER_CONTENT_LEN + PARTS_LEN;
/// Every pixel drawn opaque, its fourth byte not read: an XRGB buffer.
pub const CONTENT_OPAQUE: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Content {
    /// The handle the layer shows now, which names it.
    pub on: u64,
    pub handle: u64,
    /// Its corner on the layer.
    pub at: (u32, u32),
    pub opaque: bool,
    /// The box of it that changed, in the drawn box's pixels.
    pub changed: (u32, u32, u32, u32),
    /// The box of the surface drawn, in its pixels: all of a buffer, or a
    /// window's part of one that also holds the shadow drawn round it.
    pub view: (u32, u32, u32, u32),
}

/// The body, sixteen little-endian words, the last zero.
pub fn content_body(c: &Content) -> [u8; LAYER_CONTENT_LEN] {
    let (x, y, w, h) = c.changed;
    let (vx, vy, vw, vh) = c.view;
    let words = [
        c.on as u32,
        (c.on >> 32) as u32,
        c.handle as u32,
        (c.handle >> 32) as u32,
        c.at.0,
        c.at.1,
        if c.opaque { CONTENT_OPAQUE } else { 0 },
        x,
        y,
        w,
        h,
        vx,
        vy,
        vw,
        vh,
        0,
    ];
    let mut body = [0u8; LAYER_CONTENT_LEN];
    for (i, w) in words.iter().enumerate() {
        body[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    body
}

pub fn layer_content(port: u32, request_id: u32, c: &Content) -> Result<(), &'static str> {
    let body = content_body(c);
    let status = call_status(port, NCMP_MAGIC, OP_LAYER_CONTENT, request_id, &body)?;
    if status != 0 {
        return Err("compositor refused the layer's content");
    }
    Ok(())
}

/// The OP_LAYER_CONTENT_PARTS body: `c`'s, then `parts` of its changed box.
/// None for no part or more than MAX_PARTS.
pub fn content_parts_body(c: &Content, parts: &[Part]) -> Option<[u8; LAYER_CONTENT_PARTS_LEN]> {
    let block = parts_block(parts)?;
    let mut body = [0u8; LAYER_CONTENT_PARTS_LEN];
    body[..LAYER_CONTENT_LEN].copy_from_slice(&content_body(c));
    body[LAYER_CONTENT_LEN..].copy_from_slice(&block);
    Some(body)
}

/// OP_LAYER_CONTENT_PARTS: `c`, repainting only `parts` of its changed box.
pub fn layer_content_parts(
    port: u32,
    request_id: u32,
    c: &Content,
    parts: &[Part],
) -> Result<(), &'static str> {
    let body = content_parts_body(c, parts).ok_or("no parts to tell the compositor")?;
    let status = call_status(port, NCMP_MAGIC, OP_LAYER_CONTENT_PARTS, request_id, &body)?;
    if status != 0 {
        return Err("compositor refused the layer's content");
    }
    Ok(())
}
