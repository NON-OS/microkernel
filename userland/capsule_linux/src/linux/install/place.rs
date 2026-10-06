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

use nonos_libc::mk_debug;

use super::auth::Verified;
use alloc::vec::Vec;

use super::manifest::Manifest;
use super::place_entry::{allowed, one, Placed};
use super::program::record;
use super::tar::{walk, Kind};
use super::Why;
use crate::linux::file::visible;

/// Unpack a package's authenticated files into the store and report how
/// many landed. `name` is the package's own name, under which what it wrote
/// is kept (`package_record`) so it can be taken out again; `chosen` says it
/// is the package the person asked for, whose program is recorded so it can
/// be started later.
///
/// All or nothing: a package with a file the store would not take, or
/// links the table could not hold, is not installed. What of it was
/// written is taken out again and the install fails as `Partial`, rather
/// than reporting Installed over a program missing half its files. Its
/// program is recorded only once every file is in.
pub fn unpack(files: &Verified, name: &str, chosen: bool) -> Result<usize, Why> {
    let archive = walk(files.files());
    let mut landed: Vec<Vec<u8>> = Vec::new();
    let mut failed = 0usize;
    let mut links: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for entry in &archive.entries {
        match &entry.kind {
            Kind::File => match one(entry) {
                Placed::Landed(at) => landed.push(at),
                Placed::Failed => failed += 1,
                Placed::Refused => {}
            },
            Kind::Symlink(to) if allowed(&entry.name) => {
                links.push((visible(b"/", &entry.name), to.clone()));
            }
            // A hard link names another member, so its target is absolute.
            Kind::Hardlink(to) if allowed(&entry.name) => {
                links.push((visible(b"/", &entry.name), visible(b"/", to)));
            }
            // Directories are implied by the paths under them.
            _ => {}
        }
    }
    // A package missing files gets no links into the table either.
    let linked = match failed {
        0 => super::place_links::record(&links),
        _ => None,
    };
    let count = linked.as_ref().map(Vec::len);
    super::place_report::say(landed.len(), links.len(), count, archive.dropped);
    let Some(linked) = linked.filter(|_| failed == 0) else {
        let what = match failed {
            0 => alloc::string::String::from("the link table"),
            n => alloc::format!("{n} file(s)"),
        };
        super::place_undo::undo(&landed, &what);
        return Err(Why::Partial);
    };
    let kept = Manifest { files: landed, links: linked };
    /*
     * Installed means it can be started and taken out again: a package
     * whose record, or whose program's record, the store would not keep is
     * taken back out and fails as `Partial`, like a file that would not go
     * in, rather than reading as installed and then as never installed.
     */
    let recorded = super::package_record::write(name, &kept);
    if !recorded || (chosen && !record(name, files)) {
        if recorded {
            super::package_record::settle(name, &Manifest::default());
        }
        if !kept.links.is_empty() {
            super::place_links::unrecord(&kept.links);
        }
        super::place_undo::undo(&kept.files, "its install record");
        return Err(Why::Partial);
    }
    // Nothing persists unless asked, and never in plaintext. The store at
    // rest is not encrypted, so an install lives until the next reboot.
    let line = b"[LINUX] unserved persist: install kept in RAM, store at rest unencrypted\n";
    let _ = mk_debug(line.as_ptr(), line.len());
    Ok(kept.files.len())
}
