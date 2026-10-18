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

//! The context line as drawn, and its cut at the grid edge.

use crate::support::line;
use crate::term::context::context_line;
use crate::term::identity::USER_FALLBACK as USER;

#[test]
fn the_context_line_reads_as_the_mockup_does() {
    assert_eq!(line(b"station", b"/workspace", b"/"), "nonos@station /workspace % ls");
    assert_eq!(line(b"station", b"/home/n/work", b"/home/n"), "nonos@station ~/work % ls");
}

#[test]
fn a_line_longer_than_the_grid_is_cut_rather_than_overrunning() {
    let cwd = vec![b'x'; 200];
    let mut out = [0u8; 96];
    let n = context_line(USER, b"station", &cwd, b"", b"ls", &mut out);
    assert_eq!(n, 96);
    assert!(out.starts_with(b"nonos@station xxx"));
}
