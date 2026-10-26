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

//! The merged-/usr layout every Debian system has. /lib64, /lib, /bin and
//! /sbin are links into /usr, and a program's interpreter is named through
//! one (jq asks for /lib64/ld-linux-x86-64.so.2, which libc6 ships under
//! /usr). base-files lays these links down on a real system, and it is in no
//! program's dependency closure, so an install lays them down itself: whole
//! where the image and the built-in BusyBox have nothing beneath, else by
//! name for each program a package places (member/merged.rs).

use alloc::vec::Vec;

use crate::linux::built_in::serves;
use crate::linux::install::member::conflict::held_by_another;
use crate::linux::install::member::merged::{by_name, whole, Link, MERGED};
use crate::linux::install::package_record::others;
use crate::linux::install::place::image::IMAGE_FILES;
use crate::linux::install::place_links::{record, unrecord};
use crate::linux::install::Why;

/// Whether the built-in answers at `path`, or for programs beneath it.
fn built_in(path: &[u8]) -> bool {
    serves(path).is_some() || serves(&[path, b"/busybox"].concat()).is_some()
}

/// Add the whole links the tree may have and does not yet; one it already
/// has is kept. A whole link an earlier install laid where none may be now
/// is taken out first. A table that will not take that fails the install.
pub fn lay_out() -> Result<usize, Why> {
    let laid = whole(IMAGE_FILES, &built_in);
    let refused: Vec<Vec<u8>> =
        MERGED.iter().filter(|m| !laid.contains(m)).map(|(from, _)| from.to_vec()).collect();
    let links: Vec<Link> = laid.iter().map(|(from, to)| (from.to_vec(), to.to_vec())).collect();
    // Part of the tree, not of any package: no manifest takes them out.
    let added = unrecord(&refused).then(|| record(&links)).flatten();
    added.map(|a| a.len()).ok_or_else(|| {
        let line = b"[LINUX] the link table would not take merged /usr; nothing installed\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
        Why::PARTIAL
    })
}

/// The links by name package `pkg` gets for what it places, `files` and
/// `links`: none at a path it places itself or another package holds.
pub fn by_name_for(files: &[Vec<u8>], links: &[Link], pkg: &str) -> Vec<Link> {
    let mut placed: Vec<Vec<u8>> = files.to_vec();
    placed.extend(links.iter().map(|(at, _)| at.clone()));
    // Another record unreadable: no link is made where its files might be.
    let Some(held_elsewhere) = others(pkg) else { return Vec::new() };
    let held = |p: &[u8]| {
        placed.iter().any(|q| q == p) || held_by_another(&[p.to_vec()], &held_elsewhere).is_some()
    };
    by_name(&placed, IMAGE_FILES, &built_in, &held)
}
