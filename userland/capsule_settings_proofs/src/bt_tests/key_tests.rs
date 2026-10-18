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

//! The Bluetooth page's keys, and the letters its card notes name.

use nonos_app_skeleton::{KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_TAB, KEY_UP};

use crate::bluetooth::panel::bt_key;
use crate::bluetooth::panel::key::BtKey;
use crate::settings::schema::blocks_for;
use crate::settings::section::Section;

#[test]
fn every_letter_the_notes_name_is_a_key() {
    let pairs = [
        (b's', BtKey::Scan),
        (b'p', BtKey::Pair),
        (b'f', BtKey::Forget),
        (b'y', BtKey::Yes),
        (b'n', BtKey::No),
    ];
    let notes = blocks_for(Section::Bluetooth).iter().filter_map(|b| b.note);
    let words: Vec<&str> = notes.flat_map(|n| n.split(|c: char| !c.is_alphabetic())).collect();
    for (letter, key) in pairs {
        assert_eq!(bt_key(letter as u32), Some(key));
        assert_eq!(bt_key(letter.to_ascii_uppercase() as u32), Some(key));
        let named = (letter.to_ascii_uppercase() as char).to_string();
        assert!(words.contains(&named.as_str()), "the notes name {named}");
    }
    assert!(words.contains(&"Enter"));
    assert_eq!(bt_key(KEY_ENTER), Some(BtKey::Enter));
}

#[test]
fn digits_moving_and_leaving_keep_their_keys() {
    for d in b'0'..=b'9' {
        assert_eq!(bt_key(d as u32), Some(BtKey::Digit(d)));
    }
    assert_eq!(bt_key(KEY_BACKSPACE), Some(BtKey::Erase));
    assert_eq!(bt_key(KEY_ESC), Some(BtKey::Close));
    assert_eq!(bt_key(KEY_TAB), Some(BtKey::NextSection));
    assert_eq!(bt_key(b']' as u32), Some(BtKey::NextSection));
    assert_eq!(bt_key(b'[' as u32), Some(BtKey::PrevSection));
    assert_eq!(bt_key(KEY_UP), Some(BtKey::Up));
    assert_eq!(bt_key(KEY_DOWN), Some(BtKey::Down));
    assert_eq!(bt_key(b'w' as u32), None);
    assert_eq!(bt_key(0x1300), None);
}
