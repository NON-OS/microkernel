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

//! The Wi-Fi page's keys, the letters its card notes name, and why a scan
//! or join it was asked for does not run.

use nonos_app_skeleton::{KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_TAB, KEY_UP};

use crate::settings::wifi_key::{wifi_key, WifiKey};
use crate::settings::wifi_refusal::{
    join_refusal, scan_refusal, NOT_SCANNED, NO_DRIVER, RADIO_OFF,
};

#[test]
fn every_letter_the_notes_name_is_a_key() {
    let pairs = [
        (b'c', WifiKey::Join),
        (b'd', WifiKey::Leave),
        (b'r', WifiKey::Remember),
        (b'f', WifiKey::Forget),
        (b'w', WifiKey::Radio),
    ];
    for (letter, key) in pairs {
        assert_eq!(wifi_key(letter as u32), Some(key));
        assert_eq!(wifi_key(letter.to_ascii_uppercase() as u32), Some(key));
    }
    assert_eq!(wifi_key(KEY_ENTER), Some(WifiKey::Scan));
    assert_eq!(wifi_key(0x20), Some(WifiKey::Scan));
}

#[test]
fn moving_and_leaving_keep_their_keys() {
    assert_eq!(wifi_key(KEY_ESC), Some(WifiKey::Close));
    assert_eq!(wifi_key(KEY_TAB), Some(WifiKey::NextSection));
    assert_eq!(wifi_key(b']' as u32), Some(WifiKey::NextSection));
    assert_eq!(wifi_key(b'[' as u32), Some(WifiKey::PrevSection));
    assert_eq!(wifi_key(KEY_UP), Some(WifiKey::Up));
    assert_eq!(wifi_key(KEY_DOWN), Some(WifiKey::Down));
    assert_eq!(wifi_key(b'x' as u32), None);
    assert_eq!(wifi_key(0x1300), None);
}

#[test]
fn a_scan_with_the_switch_off_says_so() {
    assert_eq!(scan_refusal(false), Some(RADIO_OFF));
    assert_eq!(scan_refusal(true), None);
}

#[test]
fn a_join_says_the_first_thing_in_its_way() {
    assert_eq!(join_refusal(false, false, false), Some(RADIO_OFF));
    assert_eq!(join_refusal(true, false, true), Some(NO_DRIVER));
    assert_eq!(join_refusal(true, true, false), Some(NOT_SCANNED));
    assert_eq!(join_refusal(true, true, true), None);
}
