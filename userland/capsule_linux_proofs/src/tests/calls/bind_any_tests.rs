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

//! The one bind a program that reaches out makes without NET_LISTEN_LO:
//! 0.0.0.0:0, which names nothing. Any address or any port is still bind's.

use crate::linux::net::bind_any::unnamed;

#[test]
fn only_the_unnamed_bind_of_a_program_that_reaches_out_is_answered() {
    assert!(unnamed([0; 4], 0, true));
    assert!(!unnamed([0; 4], 0, false), "a program with no NET_ANON is refused as before");
    for (ip, port) in [([127, 0, 0, 1], 0), ([0; 4], 53), ([0, 0, 0, 1], 0), ([10, 0, 2, 2], 0)] {
        assert!(!unnamed(ip, port, true), "{ip:?}:{port}");
    }
}

/// bind.rs asks it before its NET_LISTEN_LO gate, and answers done changing nothing.
#[test]
fn bind_asks_it_first_and_records_nothing() {
    let src = include_str!("../../../../capsule_linux/src/linux/net/bind.rs");
    let asked = "if super::bind_any::unnamed(want.ip, want.port, reaches_out) {\n        \
                 return errno::ok(0);\n    }";
    let at = src.find(asked).expect("bind asks the unnamed rule");
    assert!(at < src.find("mask::NET_LISTEN_LO)").unwrap());
    let reaches = "crate::linux::app::granted(guest, nonos_linux_policy::mask::NET_ANON);";
    assert!(src.contains(&format!("let reaches_out = {reaches}")));
}
