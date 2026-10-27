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

//! A need named only by what provides it waits for the named needs, as
//! KeePassXC's closure showed: icu-libs needs `icu-data`, which icu-data-en
//! and icu-data-full both provide, and qt5-qtbase names icu-data-full. The
//! two hold the same file, so taking icu-data-en first failed the install.

use crate::install::index::Index;

const INDEX: &[u8] = b"P:keepassxc\nV:2.7.8-r0\nD:qt5-qtbase so:libicuuc.so.74\n\n\
P:icu-libs\nV:74.2-r1\nD:icu-data=74.2-r1\np:so:libicuuc.so.74=74.2\n\n\
P:icu-data-en\nV:74.2-r1\np:icu-data=74.2-r1\n\n\
P:icu-data-full\nV:74.2-r1\np:icu-data=74.2-r1\n\n\
P:qt5-qtbase\nV:5.15.13-r0\nD:icu-data-full\n\n\
P:needs-virtual\nV:1-r0\nD:icu-data\n\n";

/// The closure in the order the install loop (install/run.rs) takes it.
fn closure(name: &str) -> Vec<String> {
    let index = Index::parse(INDEX);
    let (mut wanted, mut later, mut done) = (vec![name.to_string()], Vec::new(), Vec::new());
    while let Some((next, exact)) = index.next_need((&mut wanted, &mut later), &done) {
        let pkg = index.resolve(&next, exact).expect("provided");
        if !done.contains(&pkg.name) {
            done.push(pkg.name.clone());
            wanted.extend(pkg.depends.iter().cloned());
        }
    }
    done
}

#[test]
fn the_provider_a_package_names_is_taken_not_the_first() {
    let done = closure("keepassxc");
    assert!(done.contains(&"icu-data-full".to_string()), "{done:?}");
    assert!(!done.contains(&"icu-data-en".to_string()), "both hold icudt74l.dat: {done:?}");
    assert_eq!(done.len(), 4, "{done:?}");
}

#[test]
fn a_virtual_need_alone_still_finds_a_provider() {
    assert_eq!(closure("needs-virtual"), ["needs-virtual", "icu-data-en"]);
}

#[test]
fn a_need_met_already_is_passed_over() {
    let index = Index::parse(INDEX);
    let done = vec!["icu-data-full".to_string()];
    assert!(index.provided_in("icu-data", &done));
    assert!(!index.provided_in("so:libicuuc.so.74", &done));
    let (mut wanted, mut later) = (vec!["icu-data".to_string()], Vec::new());
    assert_eq!(index.next_need((&mut wanted, &mut later), &done), None);
}
