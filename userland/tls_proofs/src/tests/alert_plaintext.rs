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

//! An alert read out of a record that is still in the clear.

use crate::alert::{description_in_record, ALERT};

fn record(level: u8, description: u8) -> Vec<u8> {
    vec![ALERT, 0x03, 0x03, 0x00, 0x02, level, description]
}

#[test]
fn a_plaintext_alert_gives_up_its_description() {
    // 2 is fatal, 40 is handshake_failure.
    assert_eq!(description_in_record(&record(2, 40)), Some(40));
    assert_eq!(description_in_record(&record(1, 0)), Some(0), "a warning is still readable");
    assert_eq!(description_in_record(&record(2, 48)), Some(48), "unknown_ca");
}
#[test]
fn another_record_type_is_not_an_alert() {
    let mut not_alert = record(2, 40);
    not_alert[0] = 22; // handshake
    assert_eq!(description_in_record(&not_alert), None);
    not_alert[0] = 23; // application_data
    assert_eq!(description_in_record(&not_alert), None);
}
#[test]
fn only_a_two_byte_body_is_an_alert() {
    let mut wrong = record(2, 40);
    wrong[4] = 3; // claims three bytes
    assert_eq!(description_in_record(&wrong), None);
    wrong[4] = 1;
    assert_eq!(description_in_record(&wrong), None);
}
#[test]
fn a_partial_alert_is_not_yet_a_reason() {
    let full = record(2, 40);
    for cut in 0..full.len() {
        assert_eq!(description_in_record(&full[..cut]), None, "{cut} bytes is not enough");
    }
    assert_eq!(description_in_record(&full), Some(40));
}
