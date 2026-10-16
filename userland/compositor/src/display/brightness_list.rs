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

//! F 12's brightness messages as the compositor carries them: a
//! SET_BRIGHTNESS made, and a BRIGHTNESS answer read: an i32 status, then a count and each panel's
//! identity, level now, level count and levels. Every length is checked
//! against what arrived; a status that is not 0, or a panel that runs past
//! the end, ends the list there.

use super::brightness_rule::{Panel, OP_SET_BRIGHTNESS};

/// The most panels kept from one answer.
pub const MAX_PANELS: usize = 8;

/// The panels `reply` lists into `out`; how many.
pub fn panels<'a>(reply: &'a [u8], out: &mut [Option<Panel<'a>>; MAX_PANELS]) -> usize {
    let ok = reply.get(..4).is_some_and(|s| s == [0, 0, 0, 0]);
    let Some(&count) = reply.get(4).filter(|_| ok) else { return 0 };
    let (mut at, mut n) = (5, 0);
    while n < (count as usize).min(MAX_PANELS) {
        let Some(head) = reply.get(at..at + 6) else { break };
        let len = head[5] as usize;
        let Some(levels) = reply.get(at + 6..at + 6 + len) else { break };
        let identity = u32::from_le_bytes([head[0], head[1], head[2], head[3]]);
        out[n] = Some(Panel { identity, now: head[4], levels });
        (at, n) = (at + 6 + len, n + 1);
    }
    n
}

/// SET_BRIGHTNESS for `pct` on the panel `identity` alone.
pub fn set_request(pct: u8, identity: u32) -> [u8; 6] {
    let i = identity.to_le_bytes();
    [OP_SET_BRIGHTNESS, pct, i[0], i[1], i[2], i[3]]
}
