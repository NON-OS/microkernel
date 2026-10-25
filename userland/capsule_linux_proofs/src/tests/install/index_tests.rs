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

/* Alpine v3.20's main has gvim, providing vim, ahead of vim itself: a need
 * may be met by the provider, but the listed package is only vim, whose
 * bytes the market's pin names. */
#[test]
fn the_listed_package_is_never_a_provider_of_its_name() {
    let text =
        b"P:gvim\nV:9.1-r0\np:vim=9.1-r0\n\nP:vim\nV:9.1-r0\n\nP:only\nV:1-r0\np:ghost=1\n\n";
    let index = Index::parse(text);
    let name = |p: Option<&crate::install::index::Pkg>| p.map(|p| p.name.clone());
    assert_eq!(name(index.resolve("vim", false)), Some("gvim".into()), "a need: first provider");
    assert_eq!(name(index.resolve("vim", true)), Some("vim".into()), "listed: itself");
    assert_eq!(name(index.resolve("ghost", false)), Some("only".into()));
    assert_eq!(name(index.resolve("ghost", true)), None, "nothing is named ghost");
}
