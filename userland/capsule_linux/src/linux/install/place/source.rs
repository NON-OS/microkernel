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
//! Putting a package's files into the store, under the Linux root.
//! One file of a package as its pack holds it, and the trailer beside an
//! ELF the machine mints for it.

use alloc::vec::Vec;

use nonos_libc::mk_debug;
use nonos_linux_pack::{Kind, Source};

use super::super::record::vouch::mint;
use super::super::tar::Entry;

pub(super) fn elf(e: &Entry) -> bool {
    e.body.starts_with(b"\x7fELF")
}

/// A file at its visible path, as a pack names it; a program or a script
/// is marked to run.
pub(super) fn src<'a>(at: &[u8], bytes: &'a [u8]) -> Source<'a> {
    let name = at.strip_prefix(b"/").unwrap_or(at).to_vec();
    let exec = bytes.starts_with(b"\x7fELF") || bytes.starts_with(b"#!");
    Source { name, kind: Kind::File, exec, bytes }
}

/// The trailer for an ELF, said when there is none, as vouch_elf says it.
pub(super) fn vouched(body: &[u8]) -> Option<Vec<u8>> {
    let t = mint(body);
    if t.is_none() {
        let line = b"[LINUX] installed but unvouched: no enrolled root, or may not mint\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
    t
}
