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

use super::bytes::be16;
use super::probe::{Format, Probe};

/* Walk JPEG segments to the first frame header and read its dimensions. */
pub(super) fn jpeg_dims(b: &[u8]) -> Option<Probe> {
    let mut i = 2usize;
    while i + 4 <= b.len() {
        if b[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = b[i + 1];
        /* Standalone markers carry no length payload. */
        if marker == 0xD8 || marker == 0xD9 || (0xD0..=0xD7).contains(&marker) || marker == 0x01 {
            i += 2;
            continue;
        }
        let len = be16(b, i + 2)? as usize;
        /* SOFn markers hold the frame's height then width. */
        if matches!(marker, 0xC0..=0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF) {
            let h = be16(b, i + 5)? as u32;
            let w = be16(b, i + 7)? as u32;
            return Some(Probe { format: Format::Jpeg, w, h });
        }
        i += 2 + len;
    }
    None
}
