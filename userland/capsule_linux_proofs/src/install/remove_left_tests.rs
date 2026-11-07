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

//! A partial uninstall names each step that left something behind, and a
//! whole one says nothing more.

use super::remove_left::{undone, Left};

fn left(files: usize, links: usize, record: bool, manifest: bool, home: bool) -> Left {
    Left { files, links, record, manifest, home }
}

#[test]
fn a_whole_uninstall_has_no_line() {
    assert_eq!(undone("telegram", &left(0, 0, true, true, true)), None);
}

#[test]
fn each_step_left_undone_is_named() {
    let line = undone("nonos-image-libreoffice", &left(2, 1, false, false, false)).unwrap();
    assert_eq!(
        line,
        "[LINUX] nonos-image-libreoffice: not taken out: 2 file(s), 1 link line(s), \
         its program record, its manifest, its home\n"
    );
    for (l, named) in [
        (left(3, 0, true, true, true), "3 file(s)"),
        (left(0, 4, true, true, true), "4 link line(s)"),
        (left(0, 0, false, true, true), "its program record"),
        (left(0, 0, true, false, true), "its manifest"),
        (left(0, 0, true, true, false), "its home"),
    ] {
        let line = undone("p", &l).unwrap();
        assert_eq!(line, format!("[LINUX] p: not taken out: {named}\n"), "only what stayed");
    }
}
