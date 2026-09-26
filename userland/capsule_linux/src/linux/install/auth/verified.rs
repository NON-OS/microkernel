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

//! Package bytes that something authenticated.

use alloc::vec::Vec;

/// A package's files, decompressed. Only `package::verified` makes one, so
/// bytes that did not match a signed index cannot be unpacked at all.
pub struct Verified {
    files: Vec<u8>,
}

impl Verified {
    pub(super) fn checked(files: Vec<u8>) -> Self {
        Self { files }
    }

    pub fn files(&self) -> &[u8] {
        &self.files
    }
}
