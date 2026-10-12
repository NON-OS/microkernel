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

//! The boot framebuffer's window and geometry as pure rules, which
//! `kernel_proofs` mounts and checks (L31).

const PAGE: u64 = 4096;

/// Bytes `MkBootFramebuffer` writes: six little-endian u32s.
pub(crate) const INFO_LEN: usize = 24;

/// The window a driver may map: from the page the first pixel lies in to the
/// end of the last row, as `(base, size)`. None for a window no loader should
/// hand over, which is then never published.
pub(crate) fn window(page: u64, len: u64, offset: u32) -> Option<(u64, u64)> {
    let offset = u64::from(offset);
    if page == 0 || page & (PAGE - 1) != 0 || offset >= PAGE || len <= offset {
        return None;
    }
    page.checked_add(len)?;
    Some((page, len))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Geometry {
    pub(crate) width: u32,
    pub(crate) height: u32,
    /// Bytes from one row to the next.
    pub(crate) stride: u32,
    /// The loader's code: 0 RGB, 1 BGR, 2 RGBX, 3 BGRX.
    pub(crate) format: u32,
    /// The first pixel's offset into the window.
    pub(crate) offset: u32,
    /// Width in millimetres in the low 16 bits, height in the high 16.
    pub(crate) phys_mm: u32,
}

/// `MkBootFramebuffer`'s answer, in the order the fields are declared.
pub(crate) fn encode(g: Geometry) -> [u8; INFO_LEN] {
    let words = [g.width, g.height, g.stride, g.format, g.offset, g.phys_mm];
    let mut out = [0u8; INFO_LEN];
    for (chunk, word) in out.chunks_exact_mut(4).zip(words) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out
}
