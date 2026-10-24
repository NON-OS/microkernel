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

//! What the shell says when an app, a package or an action is refused.

use crate::says::{named, not_opened, package, vfs, NO_REPLY, OFF_AT_SETUP};

/// What a toast holds (`state/toast.rs`).
const TOAST: usize = 48;

pub(super) fn line(head: &[u8], tail: &[u8]) -> Vec<u8> {
    let mut out = [0u8; TOAST];
    let n = named(head, tail, &mut out);
    out[..n].to_vec()
}

#[test]
fn an_app_that_did_not_open_is_named() {
    assert_eq!(line(b"Terminal", not_opened(-16)), b"Terminal did not open: busy, try again");
    assert_eq!(line(b"Browser", OFF_AT_SETUP), b"Browser turned off at setup");
}

#[test]
fn a_long_name_is_cut_and_the_reason_is_kept_whole() {
    let long = [b'x'; 64];
    let said = line(&long, not_opened(-16));
    assert_eq!(said.len(), TOAST);
    assert!(said.ends_with(not_opened(-16)));
}

#[test]
fn a_refused_package_is_said_in_words_that_fit_a_toast() {
    for code in [-2, -5, -11, -13, -17, -22, -71, -99] {
        let said = line(b"Package: ", package(code));
        assert!(said.starts_with(b"Package: "));
        assert!(said.ends_with(package(code)), "code {code} was cut");
        assert!(!said.iter().any(u8::is_ascii_digit), "code {code} shows a number");
    }
    assert_eq!(package(-13), b"signature or digest failed verification");
    assert_eq!(package(-17), b"already installed");
}

#[test]
fn a_refused_desktop_action_says_why() {
    assert_eq!(line(b"Could not delete: ", vfs(-39)), b"Could not delete: the folder is not empty");
    assert_eq!(line(b"Could not rename: ", vfs(-17)), b"Could not rename: that name is taken");
    assert_eq!(vfs(NO_REPLY), b"the file service did not answer");
    assert_eq!(vfs(-28), b"no space left");
    assert_eq!(vfs(-5), b"the file service refused");
}
