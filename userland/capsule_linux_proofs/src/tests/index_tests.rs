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


//! The index reader, on a record shaped the way Alpine writes one.

use crate::install::index::Index;

#[test]
fn a_record_names_its_dependencies_and_what_it_provides() {
    let text = b"C:Q1VBuPqTmRFkXS59UyXcV3OwNgKi4=\nP:foot\nV:1.0-r0\n\
D:so:libc.musl-x86_64.so.1 fontconfig>=2.14 !foot-old /bin/sh cmd:sh pc:x\np:so:libfoot.so.1=1 foot-term\n\n";
    let index = Index::parse(text);
    let pkg = index.by_name("foot").expect("foot");
    assert_eq!(pkg.depends, ["so:libc.musl-x86_64.so.1", "fontconfig"]);
    assert!(index.by_lib("libfoot.so.1").is_some());
    assert_eq!(index.by_name("foot-term").map(|p| p.name.as_str()), Some("foot"));
}
