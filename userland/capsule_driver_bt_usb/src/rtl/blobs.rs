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

//! Where the files come from: the capsule's own embedded copies, the
//! proofs' memory. Every file is checked against its pin before it is
//! used, so a copy changed after the build is refused too.

use alloc::vec::Vec;

use super::pin::Pin;
use super::refused::Refused;

pub trait Blobs {
    /// The named file's bytes, at most `max` of them; `None` when there is
    /// no such file.
    fn read(&mut self, file: &str, max: usize) -> Option<Vec<u8>>;
}

/// The pinned file, or `None` when there is none by its name. One octet
/// past the pinned length is asked for, so a longer file is seen whole
/// enough to be refused rather than cut to the right length.
pub fn pinned<B: Blobs>(blobs: &mut B, pin: &'static Pin) -> Result<Option<Vec<u8>>, Refused> {
    match blobs.read(pin.file, pin.len + 1) {
        None => Ok(None),
        Some(bytes) if pin.holds(&bytes) => Ok(Some(bytes)),
        Some(_) => Err(Refused::NotPinned(pin.file)),
    }
}
