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

use super::error::NameError;
use crate::dns::{NAME_MAX, POINTER_MASK};

/// Pointers followed for one name. The rule below already makes a loop
/// impossible; this bounds the work a long crafted chain can ask for.
const MAX_JUMPS: usize = 32;

/// A name read out of a message, pointers followed: its labels in wire form,
/// lowercased, then the root. Two names are the same name when these match.
#[derive(Clone, Copy)]
pub struct Name {
    wire: [u8; NAME_MAX],
    len: usize,
}

impl Name {
    pub fn wire(&self) -> &[u8] {
        &self.wire[..self.len]
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.wire() == other.wire()
    }
}

/*
 * The name at `start`, and the offset just past it in the message: past the
 * first pointer when it is compressed. A pointer (RFC 1035 4.1.4) must lead
 * to before where the run of labels it ends began, so each jump lands
 * strictly earlier than the last and no chain of pointers comes round again:
 * not to itself, not forward, not back into its own name. Labels are at most
 * 63 bytes (the top two bits mark a pointer; the other two prefixes are
 * reserved and refused) and the whole name, root included, at most 255.
 */
pub fn read(message: &[u8], start: usize) -> Result<(Name, usize), NameError> {
    let mut name = Name { wire: [0; NAME_MAX], len: 0 };
    let mut pos = start;
    let mut run_start = start;
    let mut end = None;
    let mut jumps = 0usize;
    loop {
        let b = *message.get(pos).ok_or(NameError::Truncated)?;
        if b & POINTER_MASK == POINTER_MASK {
            let lo = *message.get(pos + 1).ok_or(NameError::Truncated)?;
            let target = (usize::from(b & !POINTER_MASK) << 8) | usize::from(lo);
            jumps += 1;
            if jumps > MAX_JUMPS {
                return Err(NameError::LoopDetected);
            }
            if target >= run_start {
                return Err(NameError::BadPointer);
            }
            end.get_or_insert(pos + 2);
            pos = target;
            run_start = target;
            continue;
        }
        if b & POINTER_MASK != 0 {
            return Err(NameError::BadPointer);
        }
        let len = usize::from(b);
        if len == 0 {
            name.wire[name.len] = 0;
            name.len += 1;
            return Ok((name, end.unwrap_or(pos + 1)));
        }
        // Room for this label and the root that must still follow it.
        if name.len + 1 + len + 1 > NAME_MAX {
            return Err(NameError::TooLong);
        }
        let label = message.get(pos + 1..pos + 1 + len).ok_or(NameError::Truncated)?;
        name.wire[name.len] = b;
        for (slot, byte) in name.wire[name.len + 1..].iter_mut().zip(label) {
            *slot = byte.to_ascii_lowercase();
        }
        name.len += 1 + len;
        pos += 1 + len;
    }
}
