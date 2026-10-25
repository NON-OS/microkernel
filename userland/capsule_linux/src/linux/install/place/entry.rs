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

//! One file out of a package: where it lands, and whether the machine
//! says anything about it.

use alloc::vec::Vec;

use nonos_libc::mk_debug;

use crate::linux::file::{key, store_write, visible};

use super::super::tar::Entry;

/// What became of one file.
pub(super) enum Placed {
    /// Written, at this path under the Linux root.
    Landed(Vec<u8>),
    /// A name no package may write, refused by design: not a failure.
    Refused,
    /// The store would not take it: the package is not whole.
    Failed,
}

/// `held`: a program of the package a market release names, which stays
/// unstartable until its policy is bound (minted.rs).
pub(super) fn one(entry: &Entry, held: bool) -> Placed {
    if !allowed(&entry.name) {
        return Placed::Refused;
    }
    let at = visible(b"/", &entry.name);
    let mut write = |path: &[u8], body: &[u8]| store_write(&key(path), body).is_ok();
    if !super::super::market::minted::place(&at, &entry.body, held, &mut write) {
        say(b"[LINUX] a file of the package could not be written\n");
        return Placed::Failed;
    }
    super::super::record::vouch::vouch_elf(&at, &entry.body);
    Placed::Landed(at)
}

/// Not a control file, and not a proof a package would be minting for itself.
/// A link is held to the same names as a file.
pub(super) fn allowed(name: &[u8]) -> bool {
    if name.starts_with(b".") {
        return false;
    }
    if super::super::proof_names::is_proof_name(name) {
        say(b"[LINUX] refused a package writing its own proof\n");
        return false;
    }
    true
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
