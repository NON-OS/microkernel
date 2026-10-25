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

//! A file is written whole and only under a name that is free: a second
//! write to the same name is refused and leaves the first untouched, and no
//! temporary file is left behind either way.

use nsh_enrol::files::write_new;

#[test]
fn a_second_write_is_refused_and_the_first_kept() {
    let d = std::env::temp_dir().join(format!("nsh-enrol-files-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).expect("a directory");
    let f = d.join("out");
    let path = f.to_str().expect("a path");
    write_new(path, b"first", 0o600).expect("written");
    assert!(write_new(path, b"second", 0o600).is_err());
    assert_eq!(std::fs::read(&f).expect("read"), b"first");
    let left: Vec<_> = std::fs::read_dir(&d).expect("listed").collect();
    assert_eq!(left.len(), 1, "no temporary file left");
}
