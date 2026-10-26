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

//! Which home a program's /home and /root are kept in (app/home_key.rs) and
//! which paths are in it (file/private/rule.rs): a listing's home is its
//! own whatever its release, a program no listing names has its own, and
//! only /home and /root, with FILES_HOME, are ever a home.

use crate::app::home_key::home_key;
use crate::private_rule::{home_for, private_for};

#[test]
fn a_listing_s_home_is_its_own_and_outlives_its_release() {
    let (a, b) = ([1; 32], [2; 32]);
    assert_eq!(home_key(&a, &[7; 32]), home_key(&a, &[8; 32]), "a new release keeps it");
    assert_ne!(home_key(&a, &[7; 32]), home_key(&b, &[7; 32]));
}

#[test]
fn a_program_no_listing_names_has_its_own_and_never_a_listing_s() {
    let zero = [0; 32];
    assert_ne!(home_key(&zero, &[7; 32]), home_key(&zero, &[8; 32]));
    // The domains differ, so a listing key equal to a program hash is another home.
    assert_ne!(home_key(&zero, &[7; 32]), home_key(&[7; 32], &[7; 32]));
}

#[test]
fn only_home_and_root_with_files_home_are_the_home() {
    for yes in [&b"/home"[..], b"/home/age", b"/home/age/a.age", b"/root", b"/root/.gitconfig"] {
        assert!(home_for(yes, true), "{}", String::from_utf8_lossy(yes));
        assert!(!home_for(yes, false), "{}", String::from_utf8_lossy(yes));
        assert!(private_for(yes, true));
    }
    for no in [&b"/tmp/x"[..], b"/rootx", b"/homes", b"/var/tmp", b"/", b"/usr/home"] {
        assert!(!home_for(no, true), "{}", String::from_utf8_lossy(no));
    }
}
