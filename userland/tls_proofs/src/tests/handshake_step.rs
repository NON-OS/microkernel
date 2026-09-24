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

//! What the record loop does with each decrypted record.

use super::step_plain::{decided, plain};

#[test]
fn handshake_content_is_appended() {
    let body = [0x0b, 0, 0, 2, 0, 0];
    assert_eq!(decided(&plain(&body, 22, 0)).0.as_deref(), Some(&body[..]));
    assert_eq!(decided(&plain(&body, 22, 17)).0.as_deref(), Some(&body[..]));
}

#[test]
fn an_alert_stops_the_walk_and_names_itself() {
    assert_eq!(decided(&plain(&[2, 48], 21, 0)).1, Some(48), "unknown_ca");
    assert_eq!(decided(&plain(&[2, 42], 21, 0)).1, Some(42), "bad_certificate");
    assert_eq!(decided(&plain(&[1, 0], 21, 9)).1, Some(0), "a warning, past padding");
    assert_eq!(decided(&plain(&[2, 48], 21, 0)).0, None, "and contributes no messages");
}

#[test]
fn application_data_is_ignored() {
    assert_eq!(decided(&plain(b"hi", 23, 0)), (None, None));
    assert_eq!(decided(&plain(&[], 23, 4)), (None, None));
}

#[test]
fn a_malformed_alert_is_not_a_reason() {
    assert_eq!(decided(&plain(&[2], 21, 0)), (None, None), "one byte body");
    assert_eq!(decided(&plain(&[2, 48, 48], 21, 0)), (None, None), "three byte body");
    assert_eq!(decided(&plain(&[], 21, 0)), (None, None), "no body");
}

#[test]
fn padding_alone_decides_nothing() {
    assert_eq!(decided(&[]), (None, None));
    assert_eq!(decided(&[0, 0, 0, 0]), (None, None));
}
