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

//! Which names are an app's instances, built as the kernel declares them.

use crate::instance::matcher::is_instance_of;
use crate::instance::{instance_name, INSTANCE_NAME_MAX};

#[test]
fn the_base_and_its_numbered_instances_match() {
    assert!(is_instance_of(b"app.terminal", b"app.terminal"));
    assert!(is_instance_of(b"app.terminal", b"app.terminal.1"));
    assert!(is_instance_of(b"app.terminal", b"app.terminal.2"));
    assert!(is_instance_of(b"app.terminal", b"app.terminal.10"));
}

#[test]
fn a_longer_name_or_a_non_numeric_tail_does_not() {
    for name in [
        &b"app.terminal_x"[..],
        b"app.terminalfoo",
        b"app.terminal.",
        b"app.terminal.2b",
        b"app.terminal.x",
        b"app.terminal..2",
        b"app.terminal.-1",
        b"app.termina",
        b"app.terminal_x.2",
        b"app.text_editor",
    ] {
        assert!(!is_instance_of(b"app.terminal", name), "{:?}", core::str::from_utf8(name));
    }
}

#[test]
fn instance_names_are_built_as_the_kernel_declares_them() {
    let mut buf = [0u8; INSTANCE_NAME_MAX];
    assert_eq!(instance_name(b"app.terminal", 0, &mut buf), Some(&b"app.terminal"[..]));
    assert_eq!(instance_name(b"app.terminal", 2, &mut buf), Some(&b"app.terminal.2"[..]));
    assert_eq!(instance_name(b"app.browser", 12, &mut buf), Some(&b"app.browser.12"[..]));
    let long = [b'a'; INSTANCE_NAME_MAX - 1];
    assert_eq!(instance_name(&long, 1, &mut buf), None);
}
