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

//! Where the file's bytes come from: any reader that can fill a buffer from
//! an offset. A model is checked where it lies, never loaded to be checked.

pub trait ReadAt {
    /// Fill `buf` from `offset`. False if the bytes could not be read.
    fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> bool;
}

/// A file already in memory, for tests and small inputs.
impl ReadAt for &[u8] {
    fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> bool {
        let Ok(start) = usize::try_from(offset) else {
            return false;
        };
        let Some(end) = start.checked_add(buf.len()) else {
            return false;
        };
        match self.get(start..end) {
            Some(src) => {
                buf.copy_from_slice(src);
                true
            }
            None => false,
        }
    }
}
