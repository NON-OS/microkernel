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

//! Leaving the tree through a symbolic link.

use crate::fs_paths::opened;
use crate::fs_probe::{p, OUTSIDE, O_RDONLY};
use crate::report::Report;
use crate::sys::{call, OPEN, SYMLINK};

/*
 * A link may be made; what must not happen is reaching the outside file
 * through one. One points at the root and one above it, and the file is
 * opened through each: the second resolves where the first would.
 */
pub fn through_links(r: &mut Report, up: &str) {
    let _ = call(SYMLINK, [p("/\0"), p("/tmp/root\0"), 0, 0, 0, 0]);
    let _ = call(SYMLINK, [p(&format!("/{up}\0")), p("/tmp/above\0"), 0, 0, 0, 0]);
    for via in [format!("/tmp/root/{up}{OUTSIDE}\0"), format!("/tmp/above/{OUTSIDE}\0")] {
        let rc = call(OPEN, [p(&via), O_RDONLY, 0, 0, 0, 0]);
        r.check("open through a symlink out of the root", opened(rc));
    }
}
