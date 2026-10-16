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

//! Home detection and when a path is shortened to a tilde.

use crate::support::line;
use crate::term::cwd::{strip_home, HOME};
use crate::term::identity::USER_FALLBACK as USER;

#[test]
fn the_home_directory_itself_is_a_bare_tilde() {
    assert_eq!(line(b"h", b"/home/n", b"/home/n"), "nonos@h ~ % ls");
}

#[test]
fn a_path_that_merely_starts_with_home_is_not_shortened() {
    assert_eq!(line(b"h", b"/homework", b"/home"), "nonos@h /homework % ls");
    assert_eq!(strip_home(b"/homework", b"/home"), None);
}

#[test]
fn root_and_an_unset_home_shorten_nothing() {
    assert_eq!(line(b"h", b"/a", b"/"), "nonos@h /a % ls");
    assert_eq!(line(b"h", b"/a", b""), "nonos@h /a % ls");
    assert_eq!(strip_home(b"/a", b""), None);
    assert_eq!(strip_home(b"/a", b"/"), None);
}

#[test]
fn home_is_the_users_directory() {
    assert!(HOME.ends_with(USER), "HOME must end in the identity's user");
    assert_eq!(HOME.first(), Some(&b'/'));
    assert_ne!(HOME.last(), Some(&b'/'));
}
