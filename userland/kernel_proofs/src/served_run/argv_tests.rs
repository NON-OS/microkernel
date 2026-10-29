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

//! A run's argv names its package as init spawned it, with or without the
//! word a served launch gave, so the named start of SimpleX's core may ask
//! to start again with the network as the unnamed one does; nothing else
//! names a package.

use super::run_argv::package_of;

fn words(w: &[&str]) -> Vec<String> {
    w.iter().map(|s| String::from(*s)).collect()
}

#[test]
fn a_run_names_its_package_with_or_without_its_word() {
    let core = String::from("deb:nonos-simplex-core");
    assert_eq!(package_of(&words(&["run", &core])), Some(&core));
    let named = words(&["run", &core, "word", "--user-display-name=Ada"]);
    assert_eq!(package_of(&named), Some(&core), "the named start restarts with the network");
}

#[test]
fn no_other_shape_names_a_package() {
    let cases: [&[&str]; 6] = [
        &[],
        &["run"],
        &["install", "foot"],
        &["run", "foot", "cli"],
        &["run", "foot", "word"],
        &["run", "foot", "name", "x"],
    ];
    for argv in cases {
        assert_eq!(package_of(&words(argv)), None, "{argv:?}");
    }
    assert_eq!(package_of(&words(&["run", "foot", "word", "x", "y"])), None);
}
