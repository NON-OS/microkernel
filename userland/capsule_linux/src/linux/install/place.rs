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

use super::place_entry::{allowed, one};
use super::program::record;
use super::tar::{walk, Kind};
use crate::linux::file::visible;

/// Unpack a package's authenticated files into the store and report how
/// many landed. `chosen` names the package the person asked for, whose
/// program is recorded so it can be started later.
pub fn unpack(files: &Verified, chosen: Option<&str>) -> usize {
    if let Some(name) = chosen {
        record(name, files);
    }
    let archive = walk(files.files());
    let mut landed = 0usize;
    let mut links: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for entry in &archive.entries {
        match &entry.kind {
            Kind::File => landed += usize::from(one(entry)),
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
    let linked = super::place_links::record(&links);
    super::place_report::say(landed, links.len(), linked, archive.dropped);
    // Nothing persists unless asked, and never in plaintext. The store at
    // rest is not encrypted, so an install lives until the next reboot.
    let line = b"[LINUX] unserved persist: install kept in RAM, store at rest unencrypted\n";
    let _ = mk_debug(line.as_ptr(), line.len());
    landed
}
