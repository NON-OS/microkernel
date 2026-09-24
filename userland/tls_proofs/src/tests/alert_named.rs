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

//! An alert read after decryption, and the names it is reported by.

use crate::alert::{description_in_plaintext, name, ALERT};

#[test]
fn an_encrypted_alert_gives_up_its_description() {
    assert_eq!(description_in_plaintext(ALERT, &[2, 42]), Some(42), "bad_certificate");
    assert_eq!(description_in_plaintext(22, &[2, 42]), None, "a handshake message is not one");
    assert_eq!(description_in_plaintext(ALERT, &[2]), None, "a one byte body is malformed");
    assert_eq!(description_in_plaintext(ALERT, &[]), None);
}
#[test]
fn the_descriptions_that_matter_have_names() {
    assert_eq!(name(40), "handshake_failure");
    assert_eq!(name(42), "bad_certificate");
    assert_eq!(name(48), "unknown_ca");
    assert_eq!(name(70), "protocol_version");
    assert_eq!(name(0), "close_notify", "an ordinary ending, not a fault");
}
#[test]
fn an_unnamed_description_still_prints() {
    assert_eq!(name(200), "alert");
    assert!(!name(200).is_empty());
}
