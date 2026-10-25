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

//! Hostname fallback and sanitizing.

use crate::term::identity::{choose, hostname_len, user_len, HOST_FALLBACK, USER_FALLBACK as USER};

#[test]
fn an_unreachable_policy_falls_back_without_claiming_a_configured_name() {
    assert_eq!(choose(b"", HOST_FALLBACK), HOST_FALLBACK);
    assert_eq!(HOST_FALLBACK, b"nonos");
    assert_eq!(choose(b"station", HOST_FALLBACK), b"station");
    assert_eq!(choose(b"", USER), b"nonos");
}

#[test]
fn a_configured_hostname_survives_its_fixed_width_padding() {
    let mut padded = b"station".to_vec();
    padded.resize(64, 0);
    assert_eq!(hostname_len(&padded), 7);
    assert_eq!(hostname_len(b"nonos-1.local"), 13);
    assert_eq!(hostname_len(b""), 0);
}

#[test]
fn a_hostname_carrying_anything_a_hostname_cannot_is_cut_there() {
    assert_eq!(hostname_len(b"st ation"), 2);
    assert_eq!(hostname_len(b"st\nation"), 2);
    assert_eq!(hostname_len(b"\x1b[31m"), 0);
    assert_eq!((hostname_len(b"e_k"), user_len(b"e_k"), user_len(b"e k")), (1, 3, 1));
}
