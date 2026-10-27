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

//! The line a store run of a terminal program hands the Terminal.

use crate::open_arg::linux::{is_package, NAME_MAX};
use crate::open_arg::{is_path, linux_command};

#[test]
fn a_package_name_runs_as_linux_and_the_name() {
    assert_eq!(linux_command(b"vim").as_deref(), Some("run:linux vim"));
    assert_eq!(linux_command(b"py3-gpg_1.0+x").as_deref(), Some("run:linux py3-gpg_1.0+x"));
}

#[test]
fn nothing_a_shell_would_read_as_more_than_a_name_passes() {
    for bad in [
        &b""[..],
        b"vim; reboot",
        b"vim rm",
        b"a|b",
        b"$(x)",
        b"`x`",
        b"a>b",
        b"/bin/sh",
        b"../x",
        b"-rf",
        b".hidden",
        b"Vim",
        b"vim\n",
        b"vim\0",
        "vïm".as_bytes(),
    ] {
        assert!(linux_command(bad).is_none(), "{bad:?}");
    }
}

#[test]
fn a_name_past_the_bound_is_refused() {
    let at = vec![b'a'; NAME_MAX];
    assert!(is_package(&at));
    assert!(!is_package(&vec![b'a'; NAME_MAX + 1]));
}

#[test]
fn the_line_is_never_taken_for_a_path() {
    assert!(!is_path(&linux_command(b"vim").unwrap()));
}
