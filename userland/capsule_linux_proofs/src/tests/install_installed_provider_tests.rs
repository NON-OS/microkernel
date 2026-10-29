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

//! A need named only by what provides it is met by the provider an earlier
//! install placed. On KVM, Telegram's install after KeePassXC's was refused
//! icudt74l.dat: KeePassXC's qt5-qtbase names icu-data-full, Telegram's
//! closure names only `icu-data`, and the index's first provider of that is
//! icu-data-en, which holds the same file (install/index_installed.rs).

use super::install_closure_tests::get::closure::closure;
use crate::install::index::Index;

const TEXT: &[u8] = b"P:telegram-desktop\nV:5.0.1-r6\nD:qt6-qtbase\n\n\
P:qt6-qtbase\nV:6.7.2-r0\nD:so:libicuuc.so.74\n\n\
P:icu-libs\nV:74.2-r1\nD:icu-data\np:so:libicuuc.so.74=74.2\n\n\
P:icu-data-en\nV:74.2-r1\np:icu-data=74.2-r1\n\n\
P:icu-data-full\nV:74.2-r1\np:icu-data=74.2-r1\n\n\
P:keepassxc\nV:2.7.8-r0\nD:qt5-qtbase\n\n\
P:qt5-qtbase\nV:5.15.13-r0\nD:icu-data-full so:libicuuc.so.74\n\n";

fn names(name: &str, installed: &[&str]) -> Vec<String> {
    let mut index = Index::default();
    index.add(TEXT, 0);
    let installed: Vec<String> = installed.iter().map(|n| String::from(*n)).collect();
    let pkgs = closure(&index, name, &[], &installed, 64).expect("closed");
    pkgs.iter().map(|p| p.name.clone()).collect()
}

#[test]
fn an_installed_provider_is_taken_and_kept_in_the_closure() {
    let keepassxc = names("keepassxc", &[]);
    let after =
        names("telegram-desktop", &keepassxc.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(
        after.contains(&String::from("icu-data-full")),
        "kept, so its record names it: {after:?}"
    );
    assert!(!after.contains(&String::from("icu-data-en")), "both hold icudt74l.dat: {after:?}");
}

#[test]
fn with_nothing_installed_the_choice_is_as_before() {
    let alone = names("telegram-desktop", &[]);
    assert_eq!(alone, ["telegram-desktop", "qt6-qtbase", "icu-libs", "icu-data-en"]);
}

#[test]
fn a_provider_a_package_names_still_wins_over_an_installed_one() {
    let k = names("keepassxc", &["icu-data-en"]);
    assert!(k.contains(&String::from("icu-data-full")), "{k:?}");
    assert!(!k.contains(&String::from("icu-data-en")), "{k:?}");
}
