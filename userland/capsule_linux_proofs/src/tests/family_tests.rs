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

//! A family is named by the system, never guessed from the package, and
//! each one lives in a tree of its own.

use crate::family::{places, split, Family};

#[test]
fn a_prefix_names_the_family_and_is_not_part_of_the_package() {
    assert_eq!(split("deb:jq"), (Family::Debian, "jq"));
    assert_eq!(split("pacman:nmap"), (Family::Pacman, "nmap"));
    assert_eq!(split("jq"), (Family::Alpine, "jq"));
}

#[test]
fn a_bare_name_is_alpine_even_when_it_looks_like_another_family() {
    for name in ["debjq", "deb-jq", "pacman-contrib", "Deb:jq", " deb:jq"] {
        assert_eq!(split(name), (Family::Alpine, name), "{name}");
    }
}

#[test]
fn alpine_keeps_the_tree_it_always_had() {
    assert_eq!(places(Family::Alpine), (&b"/linux"[..], &b"/nonos/linux/apps"[..]));
}

#[test]
fn no_two_families_share_a_tree_or_a_record() {
    let all = [Family::Alpine, Family::Debian, Family::Pacman].map(places);
    for (i, a) in all.iter().enumerate() {
        for b in &all[i + 1..] {
            assert_ne!(a.0, b.0);
            assert_ne!(a.1, b.1);
            // Neither tree is inside another, so no guest path reaches across.
            assert!(!b.0.starts_with(&[a.0, b"/"].concat()) && !a.0.starts_with(b.0));
        }
    }
}

#[test]
fn records_sit_outside_every_tree() {
    for family in [Family::Alpine, Family::Debian, Family::Pacman] {
        let rec = places(family).1;
        for tree in [Family::Alpine, Family::Debian, Family::Pacman].map(|f| places(f).0) {
            assert!(!rec.starts_with(tree), "{:?}", family);
        }
    }
}
