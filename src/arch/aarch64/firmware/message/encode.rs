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

use super::tag::{MessageError, Tag, REQUEST, WORDS};

/// Lay `tags` out in `buf`; returns the buffer's size in bytes.
pub(crate) fn encode(tags: &[Tag], buf: &mut [u32; WORDS]) -> Result<u32, MessageError> {
    let mut at = 2usize;
    for tag in tags {
        let room = tag.room_words();
        let end = at + 3 + room;
        if end + 1 > WORDS {
            return Err(MessageError::TooLarge);
        }
        buf[at..at + 3].copy_from_slice(&[tag.id, room as u32 * 4, REQUEST]);
        buf[at + 3..end].fill(0);
        buf[at + 3..at + 3 + tag.values.len()].copy_from_slice(tag.values);
        at = end;
    }
    buf[at] = 0;
    let bytes = (at as u32 + 1) * 4;
    buf[0] = bytes;
    buf[1] = REQUEST;
    Ok(bytes)
}
