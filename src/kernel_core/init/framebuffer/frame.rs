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

//! Which part of the firmware's framebuffer the kernel maps, or why it maps
//! none. Plain numbers in, so kernel_proofs runs this file on the host.

/// Why the handoff's framebuffer was not mapped; each is said on the log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Refusal {
    Empty,
    PitchBelowRow,
    Overflow,
    SizeBelowFrame,
}

impl Refusal {
    pub(crate) fn says(self) -> &'static [u8] {
        match self {
            Refusal::Empty => b"a zero width, height, pitch or address",
            Refusal::PitchBelowRow => b"the pitch is shorter than a row of 4-byte pixels",
            Refusal::Overflow => b"pitch times height overflows",
            Refusal::SizeBelowFrame => b"FrameBufferSize is smaller than pitch times height",
        }
    }
}

/// The mapping a frame needs: from the page holding its first byte, for
/// its pitch times its height.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Frame {
    pub(crate) base: u64,
    pub(crate) offset: usize,
    pub(crate) map_len: usize,
}

/// `stride` is the row pitch in bytes, as the handoff carries it.
pub(crate) fn frame(
    ptr: u64,
    size: u64,
    width: u32,
    height: u32,
    stride: u32,
) -> Result<Frame, Refusal> {
    if width == 0 || height == 0 || stride == 0 || ptr == 0 {
        return Err(Refusal::Empty);
    }
    if (stride as u64) < width as u64 * 4 {
        return Err(Refusal::PitchBelowRow);
    }
    let frame_len = (stride as usize).checked_mul(height as usize).ok_or(Refusal::Overflow)?;
    // Only the frame is mapped: the pitch times the height, which for a 1366
    // wide panel at a 1376 pixel pitch is more than width times height, and
    // never FrameBufferSize, which some firmware reports as the whole GPU
    // aperture (256 MiB and up) and which would take most of the MMIO window.
    if size < frame_len as u64 {
        return Err(Refusal::SizeBelowFrame);
    }
    let base = ptr & !0xFFF;
    let offset = (ptr - base) as usize;
    let map_len = offset.checked_add(frame_len).ok_or(Refusal::Overflow)?;
    Ok(Frame { base, offset, map_len })
}
