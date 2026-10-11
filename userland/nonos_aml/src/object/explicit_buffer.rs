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

//! ToBuffer and ToString (sections 19.6.138 and 19.6.143).

use super::{Object, Width};
use alloc::vec::Vec;

impl Object {
    /// A string becomes its characters and a NUL; an integer its bytes.
    pub fn to_buffer_explicit(&self, w: Width) -> Option<Vec<u8>> {
        let mut bytes = self.as_bytes(w)?;
        if matches!(self, Object::String(_)) {
            bytes.push(0);
        }
        Some(bytes)
    }
}

/// ToString: a buffer's bytes up to its first NUL or `len`, whichever is
/// first; Ones as the length means no limit.
pub fn buffer_to_string(bytes: &[u8], len: u64) -> Vec<u8> {
    let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    bytes[..end.min(usize::try_from(len).unwrap_or(usize::MAX))].to_vec()
}
