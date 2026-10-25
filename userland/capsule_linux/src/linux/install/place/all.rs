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

use super::super::auth::Verified;
use alloc::vec::Vec;

use super::super::manifest::Manifest;
use super::super::tar::{walk, Kind};
use super::super::Why;
use super::entry::{allowed, one, Placed};
use crate::linux::file::visible;

/// Unpack a package's authenticated files into the store and report how
/// many landed. `name` is the package's own name, under which what it wrote
/// is kept (`package_record`) so it can be taken out again; `chosen` says it
/// is the package the person asked for, whose program is recorded so it can
/// be started later; `merged`, that it is Debian's (deb/merged_usr.rs).
///
/// All or nothing: a package with a file the store would not take, or
/// links the table could not hold, is not installed. What of it was written
/// is taken out again and the install fails as `Partial`. Its program is
/// recorded only once every file is in.
pub fn unpack(files: &Verified, name: &str, chosen: bool, merged: bool) -> Result<usize, Why> {
    let archive = walk(files.files());
    super::check::check(&archive.entries, name)?;
    let mut landed: Vec<Vec<u8>> = Vec::new();
    let mut failed = 0usize;
    let mut links: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for entry in &archive.entries {
        match &entry.kind {
            Kind::File => match one(entry, chosen && super::super::market::hold::held()) {
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
    if merged {
        links.extend(super::super::deb::merged_usr::by_name_for(&landed, &links, name));
    }
    // A package missing files gets no links into the table either.
    let linked = match failed {
        0 => super::super::place_links::record(&links),
        _ => None,
    };
    let count = linked.as_ref().map(Vec::len);
    super::report::say(landed.len(), links.len(), count, archive.dropped);
    let Some(linked) = linked.filter(|_| failed == 0) else {
        return Err(super::undo::partial(&landed, failed));
    };
    super::keep::keep(files, name, chosen, Manifest { files: landed, links: linked })
}
