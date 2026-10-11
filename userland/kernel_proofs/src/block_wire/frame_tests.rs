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

use super::frame::frame;
use super::header::{HDR_LEN, MAGIC, OP_READ, VERSION};

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

#[test]
fn the_header_carries_the_unit_where_the_shared_one_has_its_reserved_word() {
    let body = [7u8; 12];
    let f = frame(OP_READ, 3, 0xdead_beef, &body);
    assert_eq!(f.len(), HDR_LEN + body.len());
    assert_eq!(u32_at(&f, 0), MAGIC);
    assert_eq!(u16_at(&f, 4), VERSION);
    assert_eq!(u16_at(&f, 6), OP_READ);
    assert_eq!(u16_at(&f, 8), 0, "no flags");
    assert_eq!(u16_at(&f, 10), 3, "unit");
    assert_eq!(u32_at(&f, 12), 0xdead_beef);
    assert_eq!(u32_at(&f, 16), body.len() as u32);
    assert_eq!(&f[HDR_LEN..], &body);
}

#[test]
fn unit_zero_frames_like_the_shared_header() {
    let f = frame(OP_READ, 0, 1, &[]);
    assert_eq!(&f[10..12], &[0, 0]);
    assert_eq!(f.len(), HDR_LEN);
}
