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

//! A package that landed whole, kept: its record, and its program's when it
//! is the one asked for. Either refused takes all of it back out (`all.rs`).

use nonos_libc::mk_debug;

use super::super::auth::Verified;
use super::super::manifest::Manifest;
use super::super::record::program::record;
use super::super::Why;

pub(super) fn keep(
    files: &Verified,
    name: &str,
    chosen: bool,
    kept: Manifest,
) -> Result<usize, Why> {
    /*
     * Installed means it can be started and taken out again: a package
     * whose record, or whose program's record, the store would not keep is
     * taken back out and fails as `Partial`, like a file that would not go
     * in, rather than reading as installed and then as never installed.
     */
    let recorded = super::super::package_record::write(name, &kept);
    if !recorded || (chosen && !record(name, files)) {
        if recorded {
            super::super::package_record::settle(name, &Manifest::default());
        }
        if !kept.links.is_empty() {
            super::super::place_links::unrecord(&kept.links);
        }
        super::undo::undo(&kept.files, "its install record");
        return Err(Why::PARTIAL);
    }
    // Nothing persists unless asked, and never in plaintext. The store at
    // rest is not encrypted, so an install lives until the next reboot.
    let line = b"[LINUX] unserved persist: install kept in RAM, store at rest unencrypted\n";
    let _ = mk_debug(line.as_ptr(), line.len());
    Ok(kept.files.len())
}
