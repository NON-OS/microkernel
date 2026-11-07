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

//! OP_PRESENT_LAYER_PARTS and OP_LAYER_CONTENT_PARTS as the compositor
//! takes them: the plain op, then the boxes it was told apart into, each
//! non-empty and wholly inside its changed box or the whole is refused,
//! read here apart from the compositor's own decoder.

use super::{call, layer_content, present_layer, Call, Content};
use crate::swap::{Part, MAX_PARTS, PARTS_LEN, PRESENT_LAYER_LEN, PRESENT_LAYER_PARTS_LEN};

/// The boxes of a parts block when each lies inside `bound`, x, y, width
/// and height; None for a block the compositor refuses.
fn told(block: &[u8], bound: Part) -> Option<Vec<Part>> {
    let word = |i: usize| {
        let at = i * 4;
        u32::from_le_bytes([block[at], block[at + 1], block[at + 2], block[at + 3]])
    };
    if block.len() != PARTS_LEN {
        return None;
    }
    let n = usize::try_from(word(0)).ok().filter(|n| (1..=MAX_PARTS).contains(n))?;
    let (bx, by, bw, bh) = bound.map_wide();
    let mut out = Vec::new();
    for i in 0..MAX_PARTS {
        let p = (word(1 + i * 4), word(2 + i * 4), word(3 + i * 4), word(4 + i * 4));
        if i >= n {
            (p == (0, 0, 0, 0)).then_some(())?;
            continue;
        }
        let (x, y, w, h) = p.map_wide();
        let inside = w > 0 && h > 0 && x >= bx && y >= by && x + w <= bx + bw && y + h <= by + bh;
        inside.then_some(())?;
        out.push(p);
    }
    Some(out)
}

trait Wide {
    fn map_wide(self) -> (u64, u64, u64, u64);
}

impl Wide for Part {
    fn map_wide(self) -> (u64, u64, u64, u64) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into())
    }
}

pub fn present_layer_parts(
    p: u32,
    r: u32,
    body: &[u8; PRESENT_LAYER_PARTS_LEN],
) -> Result<(), &'static str> {
    let word = |at: usize| u32::from_le_bytes([body[at], body[at + 1], body[at + 2], body[at + 3]]);
    let area = (word(16), word(20), word(24), word(28));
    let (head, block) = body.split_at(PRESENT_LAYER_LEN);
    let parts = told(block, area).ok_or("compositor rejected present")?;
    let head: &[u8; PRESENT_LAYER_LEN] = head.try_into().map_err(|_| "a present's length")?;
    present_layer(p, r, head)?;
    call(Call::Parts(parts))
}

pub fn layer_content_parts(
    p: u32,
    r: u32,
    c: &Content,
    parts: &[Part],
) -> Result<(), &'static str> {
    let block = crate::swap::parts_block(parts).ok_or("no parts to tell the compositor")?;
    let parts = told(&block, c.changed).ok_or("compositor refused the layer's content")?;
    layer_content(p, r, c)?;
    call(Call::Parts(parts))
}
