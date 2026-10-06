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

//! The ID3v1 block: the last 128 bytes of an MP3, starting "TAG".
//!
//! It is the oldest tag and the poorest (30 Latin-1 bytes per name, no album
//! artist, no cover), but plenty of older rips carry nothing else, so Music
//! still reads it to fill what an ID3v2 tag did not name. ID3v1.1 keeps the
//! track number in the comment's last byte when the byte before it is NUL.

use super::text::{latin1, tidy, year};
use super::Tags;

const BLOCK: usize = 128;

/// Reads the v1 block from the end of `tail`; anything else gives empty tags.
pub fn read(tail: &[u8]) -> Tags {
    let Some(start) = tail.len().checked_sub(BLOCK) else {
        return Tags::default();
    };
    let b = &tail[start..];
    if &b[..3] != b"TAG" {
        return Tags::default();
    }
    let comment = &b[97..127];
    let track = (comment[28] == 0 && comment[29] != 0).then(|| u16::from(comment[29]));
    Tags {
        title: tidy(latin1(&b[3..33])),
        artist: tidy(latin1(&b[33..63])),
        album: tidy(latin1(&b[63..93])),
        album_artist: None,
        track,
        year: year(&latin1(&b[93..97])),
        cover: None,
    }
}
