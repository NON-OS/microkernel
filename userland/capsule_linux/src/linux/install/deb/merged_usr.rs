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
//! program's dependency closure, so an install lays them down itself.

use alloc::vec::Vec;

use crate::linux::install::place_links::record;

const MERGED: [(&[u8], &[u8]); 4] = [
    (b"/bin", b"usr/bin"),
    (b"/sbin", b"usr/sbin"),
    (b"/lib", b"usr/lib"),
    (b"/lib64", b"usr/lib64"),
];

/// Add the links the tree does not have yet; one it already has is kept.
pub fn lay_out() -> Option<usize> {
    let links: Vec<(Vec<u8>, Vec<u8>)> =
        MERGED.iter().map(|(from, to)| (from.to_vec(), to.to_vec())).collect();
    record(&links)
}
