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

/// A bounds-checked big-endian reader over a WOFF2 file, table or stream.
pub(super) struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(data: &'a [u8]) -> Cursor<'a> {
        Cursor { data, pos: 0 }
    }

    pub(super) fn bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.data.get(self.pos..self.pos.checked_add(n)?)?;
        self.pos += n;
        Some(s)
    }

    pub(super) fn u8(&mut self) -> Option<u8> {
        Some(self.bytes(1)?[0])
    }

    pub(super) fn u16(&mut self) -> Option<u16> {
        let b = self.bytes(2)?;
        Some(u16::from_be_bytes([b[0], b[1]]))
    }

    pub(super) fn u32(&mut self) -> Option<u32> {
        let b = self.bytes(4)?;
        Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// UIntBase128 (WOFF2 6.1.1): seven bits a byte, high bit set on all
    /// but the last, at most five bytes, no leading zero digit, fits u32.
    pub(super) fn base128(&mut self) -> Option<u32> {
        let mut v = 0u32;
        for i in 0..5 {
            let b = self.u8()?;
            if (i == 0 && b == 0x80) || v & 0xfe00_0000 != 0 {
                return None;
            }
            v = v << 7 | (b & 0x7f) as u32;
            if b & 0x80 == 0 {
                return Some(v);
            }
        }
        None
    }

    /// 255UInt16 (WOFF2 6.1.1): one byte below 253, else an escape code.
    pub(super) fn u255(&mut self) -> Option<u16> {
        match self.u8()? {
            253 => self.u16(),
            254 => Some(self.u8()? as u16 + 506),
            255 => Some(self.u8()? as u16 + 253),
            c => Some(c as u16),
        }
    }
}
