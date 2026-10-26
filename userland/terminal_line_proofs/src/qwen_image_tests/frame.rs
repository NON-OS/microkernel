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

//! The frame a vision tier's image goes in, and the room it leaves.

use super::super::image::{frame, image_max, MAGIC};

#[test]
fn the_frame_is_the_magic_the_length_and_the_bytes() {
    let f = frame(b"\x89PNG");
    assert_eq!(&f[..8], MAGIC);
    assert_eq!(&f[8..12], &4u32.to_le_bytes());
    assert_eq!(&f[12..], b"\x89PNG");
    assert_eq!(frame(b""), [&MAGIC[..], &[0, 0, 0, 0]].concat(), "no image is an empty frame");
}

#[test]
fn the_image_leaves_room_for_its_frame_and_the_question() {
    let q = b"What is this?";
    assert_eq!(image_max(1 << 20, q), (1 << 20) - 12 - q.len() - 1);
    assert_eq!(image_max(10, q), 0);
}
