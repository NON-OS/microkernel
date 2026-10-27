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

use super::who::{shown, GUEST};

#[test]
fn a_guest_is_only_foreign_on_a_console_without_capsule_output() {
    for name in ["foreign:monero-wallet", "foreign:simplex-chat", "foreign:x", "foreign:"] {
        assert_eq!(shown(name.as_bytes(), false), b"foreign", "{name}");
    }
}

#[test]
fn a_development_console_keeps_the_whole_name() {
    assert_eq!(shown(b"foreign:monero-wallet", true), b"foreign:monero-wallet");
}

#[test]
fn a_capsule_keeps_its_name_everywhere() {
    for name in ["app.linux.run", "vfs", "net.anon", "foreigner", "foreign"] {
        assert_eq!(shown(name.as_bytes(), false), name.as_bytes(), "{name}");
    }
}

#[test]
fn the_marker_is_the_one_guest_names_are_made_with() {
    let made = include_str!("../../../../src/process/foreign/guest_name.rs");
    let marker = core::str::from_utf8(GUEST).unwrap();
    assert!(made.contains(&format!("\"{marker}{{}}\"")), "guest_name.rs no longer writes {marker}");
}
