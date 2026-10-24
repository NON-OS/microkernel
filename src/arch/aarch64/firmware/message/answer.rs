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

use super::tag::{MessageError, Tag, ANSWERED, TAG_ANSWERED, WORDS};

/// The answer to the `index`th tag of `tags`, as encoded into `buf`.
pub(crate) fn answer<'b>(
    tags: &[Tag],
    buf: &'b [u32; WORDS],
    index: usize,
) -> Result<&'b [u32], MessageError> {
    if buf[1] != ANSWERED {
        return Err(MessageError::NotAnswered);
    }
    let mut at = 2usize;
    for (i, tag) in tags.iter().enumerate() {
        let room = tag.room_words();
        if i == index {
            let code = buf[at + 2];
            let words = ((code & !TAG_ANSWERED) as usize).div_ceil(4);
            if code & TAG_ANSWERED == 0 || words > room {
                return Err(MessageError::TagNotAnswered);
            }
            return Ok(&buf[at + 3..at + 3 + words]);
        }
        at += 3 + room;
    }
    Err(MessageError::TagNotAnswered)
}
