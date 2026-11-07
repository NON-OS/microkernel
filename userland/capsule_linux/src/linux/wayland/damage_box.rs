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

//! How a box of a client's buffer is read out of the guest, into a picture
//! laid out as the buffer is. Outside the box it said changed a client's
//! buffer may hold an older frame, as a client drawing into two buffers in
//! turn redraws only what changed, so only the box changes in the picture.
//! A box as wide as the buffer's stride is one read; any other is its rows,
//! read at the stride in as few kernel calls as they fit (guest/mem_rows.rs),
//! and no byte between them is read or touched. Pure, so the host proofs
//! hold it.

/// A box in buffer pixels, corner to corner: x0, y0, x1, y1.
pub type Corners = (u32, u32, u32, u32);

/// `count` reads of `len` bytes, the first at `first` and each next `step`
/// bytes on, as offsets from the buffer's first pixel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reads {
    pub first: usize,
    pub step: usize,
    pub len: usize,
    pub count: usize,
}

impl Reads {
    /// Where read `i` starts; None past the last.
    pub fn at(&self, i: usize) -> Option<usize> {
        if i >= self.count {
            return None;
        }
        self.first.checked_add(self.step.checked_mul(i)?)
    }

    /// The bytes all of them take.
    pub fn bytes(&self) -> usize {
        self.len.saturating_mul(self.count)
    }

    /// The bytes from the first read's start to the last one's end.
    pub fn span(&self) -> Option<usize> {
        self.step.checked_mul(self.count.checked_sub(1)?)?.checked_add(self.len)
    }
}

/// How to read box `b` of a `w` by `h` buffer at `stride` bytes a row, of
/// which the first `read` bytes are the client's (present/source.rs): one
/// read when its rows meet, else its rows, each `stride` bytes after the
/// one before. None when the box is empty or not wholly on the buffer, a
/// row of the buffer is wider than its stride, or a read would end past
/// `read`.
pub fn plan((w, h, stride): (u32, u32, u32), read: usize, b: Corners) -> Option<Reads> {
    let (x0, y0, x1, y1) = b;
    if x0 >= x1 || y0 >= y1 || x1 > w || y1 > h {
        return None;
    }
    let (stride, width) = (stride as usize, (w as usize).checked_mul(4)?);
    if width > stride {
        return None;
    }
    let (row, count) = (((x1 - x0) as usize).checked_mul(4)?, (y1 - y0) as usize);
    let first = (y0 as usize).checked_mul(stride)?.checked_add((x0 as usize).checked_mul(4)?)?;
    let len = stride.checked_mul(count - 1)?.checked_add(row)?;
    if first.checked_add(len)? > read {
        return None;
    }
    Some(match row == stride {
        true => Reads { first, step: 0, len, count: 1 },
        false => Reads { first, step: stride, len: row, count },
    })
}
