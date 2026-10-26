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

//! What fails a package before any of it is written: a member name that is
//! malformed or the personality's own (member/name.rs), one that takes the
//! place of the image's own files (member/image.rs), or a path another
//! installed package holds (member/conflict.rs). Said with the member and
//! the reason; the install fails as Package and nothing of it lands.

use alloc::format;
use alloc::vec::Vec;

use nonos_libc::mk_debug;

use super::super::member::conflict::held_by_another;
use super::super::member::image::shadows;
use super::super::member::name::member_refused;
use super::super::proof_names::is_proof_name;
use super::super::tar::{Entry, Kind};
use super::super::Why;
use super::image::IMAGE_FILES;
use crate::linux::file::visible;

pub(super) fn check(entries: &[Entry], pkg: &str) -> Result<(), Why> {
    let mut mine: Vec<Vec<u8>> = Vec::new();
    for e in entries {
        let target = match &e.kind {
            Kind::Hardlink(to) => Some(to.as_slice()),
            _ => None,
        };
        for named in core::iter::once(e.name.as_slice()).chain(target) {
            if let Some(why) = member_refused(named) {
                return refuse(pkg, named, why.about());
            }
        }
        // The names entry.rs passes over silently are not this package's.
        let placed = !e.name.starts_with(b".") && !is_proof_name(&e.name);
        if placed && !matches!(e.kind, Kind::Dir) {
            let at = visible(b"/", &e.name);
            if shadows(&at, matches!(e.kind, Kind::Symlink(_)), IMAGE_FILES) {
                return refuse(pkg, &e.name, "the image carries it, or a file under it");
            }
            mine.push(at);
        }
    }
    let Some(others) = super::super::package_record::others(pkg) else {
        return refuse(pkg, b"", "another package's record is unreadable");
    };
    match held_by_another(&mine, &others) {
        Some(at) => refuse(pkg, at, "another installed package holds it"),
        None => Ok(()),
    }
}

fn refuse(pkg: &str, member: &[u8], why: &str) -> Result<(), Why> {
    let member = alloc::string::String::from_utf8_lossy(member);
    let line = format!("[LINUX] {pkg}: refused member {member}: {why}; nothing of it is written\n");
    let _ = mk_debug(line.as_ptr(), line.len());
    Err(Why::PACKAGE)
}
