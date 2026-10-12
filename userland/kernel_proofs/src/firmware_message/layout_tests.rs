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

use super::message::answer::answer;
use super::message::encode::encode;
use super::message::tag::{MessageError, Tag, WORDS};

const ANSWERED: u32 = 0x8000_0000;

#[test]
fn several_tags_are_found_by_index_whatever_their_room() {
    let tags = [
        Tag { id: 0x0004_8003, values: &[1920, 1080], room: 2 },
        Tag { id: 0x0004_8005, values: &[32], room: 1 },
        Tag { id: 0x0004_0001, values: &[4096], room: 2 },
        Tag { id: 0x0004_0008, values: &[], room: 1 },
    ];
    let mut buf = [0; WORDS];
    // 2 header words, 5 + 4 + 5 + 4 tag words, the end tag.
    assert_eq!(encode(&tags, &mut buf), Ok(21 * 4));
    assert_eq!(buf[20], 0);
    buf[1] = ANSWERED;
    for (code, values) in
        [(4, &[1920u32, 1080][..]), (9, &[32]), (13, &[0xfe00_0000, 8_294_400]), (18, &[7680])]
    {
        buf[code] = ANSWERED | (values.len() as u32 * 4);
        buf[code + 1..code + 1 + values.len()].copy_from_slice(values);
    }
    assert_eq!(answer(&tags, &buf, 2), Ok(&[0xfe00_0000, 8_294_400][..]));
    assert_eq!(answer(&tags, &buf, 3), Ok(&[7680][..]));
}

#[test]
fn a_message_past_the_buffer_is_refused_not_cut() {
    let big = [Tag { id: 1, values: &[], room: WORDS }];
    assert_eq!(encode(&big, &mut [0; WORDS]), Err(MessageError::TooLarge));
    // Exactly full: header, tag header, room, end tag.
    let fits = [Tag { id: 1, values: &[], room: WORDS - 6 }];
    assert_eq!(encode(&fits, &mut [0; WORDS]), Ok(WORDS as u32 * 4));
}
