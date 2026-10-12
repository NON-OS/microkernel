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

use crate::arch::fdt::error::FdtError;

use super::format::PixelFormat;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SimpleFramebuffer {
    /// CPU physical address of the first pixel, and the bytes `reg` covers.
    pub base: u64,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    /// Bytes from one row to the next.
    pub stride: u32,
    pub format: PixelFormat,
}

/// Why a node that is there is not used: drawing into a misdescribed
/// framebuffer writes past it, so each of these is a refusal, not a guess.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    Tree(FdtError),
    /// `reg`, `width`, `height`, `stride` or `format` is absent or not one cell.
    Missing,
    UnknownFormat,
    /// A row narrower than its pixels, or rows that run past `reg`.
    Geometry,
}

impl From<FdtError> for Refused {
    fn from(e: FdtError) -> Self {
        Refused::Tree(e)
    }
}
