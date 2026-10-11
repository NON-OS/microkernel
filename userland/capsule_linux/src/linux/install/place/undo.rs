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

//! Taking a package that did not land whole back out of the store.
//!
//! Each file written for it goes, with the trailer minted beside a program,
//! so nothing of a half-installed package can be started or vouched for.
//! Packages of the same install that landed whole before it stay: each is a
//! complete package on its own. A path the package overwrote held the new
//! bytes by then, so taking it out loses nothing the old file still had.

use alloc::format;
use alloc::vec::Vec;

use nonos_libc::mk_debug;

use crate::linux::attest_paths::beside;
use crate::linux::file::{key, store_unlink};

/// `why` says what was not written: files, or the link table.
pub(super) fn undo(landed: &[Vec<u8>], why: &str) {
    let mut left = 0usize;
    for at in landed {
        if store_unlink(&key(at)).is_err() {
            left += 1;
        }
        // Only a program has one; for any other file there is nothing here.
        let _ = store_unlink(&key(&beside(at, b".zk_trailer.bin")));
    }
    let line = format!(
        "[LINUX] install failed, {why} not written: took out {} of {} file(s), {left} would not go\n",
        landed.len() - left,
        landed.len()
    );
    let _ = mk_debug(line.as_ptr(), line.len());
}
