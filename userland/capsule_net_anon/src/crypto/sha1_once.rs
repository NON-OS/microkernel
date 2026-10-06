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

//! SHA-1 of one buffer, for the directory's document and key digests.

use super::sha1::Sha1;

/// The SHA-1 of `data`.
///
pub fn sha1_digest(data: &[u8]) -> [u8; 20] {
    let mut hash = Sha1::new();
    hash.update(data);
    hash.finish()
}
