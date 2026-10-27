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

//! Alpine's install_if, with the compositor's name counted as the system's,
//! on records written the way Alpine writes v3.20's.

use crate::install::index::Index;
use crate::install::install_if::{next, triggered};

const INDEX: &[u8] = b"P:qt6-qtbase-x11\nV:6.6.3-r0\n\n\
P:qt6-qtwayland\nV:6.6.3-r0\ni:wayland-libs-server qt6-qtbase-x11\n\n\
P:zathura\nV:0.5.6-r0\n\n\
P:zathura-doc\nV:0.5.6-r0\ni:docs zathura=0.5.6-r0\n\n\
P:imv-wayland\nV:4.5.0-r0\ni:imv=4.5.0-r0 wayland-libs-server\n\n\
P:plain\nV:1-r0\n\n";

fn names(done: &[&str]) -> Vec<String> {
    let index = Index::parse(INDEX);
    let done: Vec<String> = done.iter().map(|s| s.to_string()).collect();
    triggered(&index, &done).iter().map(|p| p.name.clone()).collect()
}

#[test]
fn a_qt_program_brings_qts_wayland_platform() {
    assert_eq!(names(&["keepassxc", "qt6-qtbase-x11"]), ["qt6-qtwayland"]);
}

#[test]
fn a_rule_naming_what_is_not_installed_brings_nothing() {
    assert!(names(&["zathura"]).is_empty(), "docs is never installed here");
    assert!(names(&["plain"]).is_empty());
}

#[test]
fn the_systems_name_alone_brings_nothing() {
    assert!(names(&[]).is_empty());
    assert!(names(&["libfoo"]).is_empty());
}

#[test]
fn what_is_installed_already_is_not_brought_again() {
    assert!(names(&["qt6-qtbase-x11", "qt6-qtwayland"]).is_empty());
    let index = Index::parse(INDEX);
    let done = vec!["imv".to_string()];
    assert_eq!(next(&index, &done).as_deref(), Some("imv-wayland"), "version constraint dropped");
    assert_eq!(index.by_name("qt6-qtwayland").map(|p| p.install_if.len()), Some(2));
}
