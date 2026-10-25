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
//! Each file written for it goes, with what was written beside it
//! (minted.rs), so nothing of a half-installed package can be started or
//! vouched for.
//! Packages of the same install that landed whole before it stay: each is a
//! complete package on its own. A path the package overwrote held the new
//! bytes by then, so taking it out loses nothing the old file still had.

use alloc::format;
use alloc::vec::Vec;

use nonos_libc::mk_debug;

use super::super::market::take::take_out;
use super::super::Why;
use crate::linux::file::{key, store_found, store_unlink};

/// `why` says what was not written: files, or the link table.
pub(super) fn undo(landed: &[Vec<u8>], why: &str) {
    let mut unlink = |p: &[u8]| {
        let _ = store_unlink(&key(p));
    };
    let mut found = |p: &[u8]| store_found(&key(p));
    let left = landed.iter().filter(|at| !take_out(at, &mut unlink, &mut found)).count();
    let line = format!(
        "[LINUX] install failed, {why} not written: took out {} of {} file(s), {left} would not go\n",
        landed.len() - left,
        landed.len()
    );
    let _ = mk_debug(line.as_ptr(), line.len());
}

/// The install's failure once `failed` files, or the link table, would not
/// go: what landed is taken out, and the package is Partial.
pub(super) fn partial(landed: &[Vec<u8>], failed: usize) -> Why {
    let what = match failed {
        0 => alloc::string::String::from("the link table"),
        n => format!("{n} file(s)"),
    };
    undo(landed, &what);
    Why::Partial
}
