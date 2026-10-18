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

//! The logo BGRT names, copied and checked, with no pointer read: the
//! caller hands a reader of physical memory (the kernel's directmap, or
//! the proofs' bytes). The BMP's headers are read and checked first, and
//! nothing more is read unless they pass and the file is no larger than
//! `MAX_IMAGE`; then the whole file is read and checked again, and kept
//! only when both checks agree. One copy for every architecture: x86's
//! `bgrt_load` and the aarch64 tables mount it.

use super::bgrt_entry::{bgrt, Bgrt, IMAGE_BMP};
use super::bmp_check::{bmp, Bmp, FILE_HEADER};
use alloc::vec::Vec;

pub const MAX_IMAGE: u32 = 16 << 20;
const HEADERS: usize = FILE_HEADER + 40;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogoError {
    /// No BGRT that reads, or an image that is not a BMP.
    NoLogo,
    /// The memory the table names could not be read.
    Unreadable,
    /// The bytes there are not a BMP the splash can draw.
    NotDrawable,
    TooLarge,
}

pub type Logo = (Bgrt, Bmp, Vec<u8>);

pub fn copy_logo(
    table: &[u8],
    mut read: impl FnMut(u64, usize) -> Option<Vec<u8>>,
) -> Result<Logo, LogoError> {
    let b = bgrt(table).filter(|b| b.image_type == IMAGE_BMP).ok_or(LogoError::NoLogo)?;
    let head = read(b.address, HEADERS).ok_or(LogoError::Unreadable)?;
    let h = bmp(&head, false).ok_or(LogoError::NotDrawable)?;
    if h.file_size > MAX_IMAGE {
        return Err(LogoError::TooLarge);
    }
    let bytes = read(b.address, h.file_size as usize).ok_or(LogoError::Unreadable)?;
    match bmp(&bytes, true) {
        Some(m) if m == h => Ok((b, m, bytes)),
        _ => Err(LogoError::NotDrawable),
    }
}
