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

//! Member paths, as every archive family writes them.

use alloc::vec::Vec;

/// A member path as the archive's root sees it. dpkg writes `./usr/bin/x`
/// where apk writes `usr/bin/x`, and a directory ends in `/` on the wire.
/// A symlink's target is left as written: `./` there is meaningful.
pub(super) fn member(mut name: Vec<u8>) -> Vec<u8> {
    while name.starts_with(b"./") {
        name.drain(..2);
    }
    while name.len() > 1 && name.last() == Some(&b'/') {
        name.pop();
    }
    name
}
