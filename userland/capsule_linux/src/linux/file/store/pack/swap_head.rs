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

//! A held pack already in place. A swap that finds the old pack held open
//! leaves it serving; when that pack begins as the new one does, its header
//! and index name every file with its size, place and digest, so it holds
//! these very files and the package is installed as asked. Installing a
//! second app put each shared library again, and one an open app held
//! failed the whole install, though nothing in it had changed.

use super::shelf::{Held, Shelf};

/// Ok when `why` is a hold and the pack at `at` begins with `pack`'s first
/// `head` bytes, its header and index; otherwise `why` stands. No head
/// (`head` 0, or past `pack`'s end) is never the same.
pub fn in_place(
    s: &mut impl Shelf,
    at: &[u8],
    pack: &[u8],
    head: usize,
    why: Held,
) -> Result<(), Held> {
    let index = pack.get(..head).filter(|_| head > 0 && why == Held::InUse);
    match index.is_some_and(|i| s.head(at, head).as_deref() == Some(i)) {
        true => Ok(()),
        false => Err(why),
    }
}
