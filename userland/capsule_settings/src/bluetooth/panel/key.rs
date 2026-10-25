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

//! What a key does on the Bluetooth page. Arrows walk the device list; the
//! letters are the ones the page's card notes name, so this table and those
//! notes are one list. Digits type a passkey the device shows.

use nonos_app_skeleton::{KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_TAB, KEY_UP};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BtKey {
    Close,
    NextSection,
    PrevSection,
    Up,
    Down,
    /// Scanning on or off.
    Scan,
    /// Let one new device pair.
    Pair,
    /// Forget the highlighted device.
    Forget,
    Digit(u8),
    Erase,
    /// Send the digits typed.
    Enter,
    /// Both sides show the same number.
    Yes,
    /// They do not, or pairing is declined.
    No,
}

pub fn bt_key(code: u32) -> Option<BtKey> {
    let key = match code {
        KEY_ESC => BtKey::Close,
        KEY_TAB => BtKey::NextSection,
        KEY_UP => BtKey::Up,
        KEY_DOWN => BtKey::Down,
        KEY_ENTER => BtKey::Enter,
        KEY_BACKSPACE => BtKey::Erase,
        _ => match u8::try_from(code).map(|c| c.to_ascii_lowercase()) {
            Ok(b']') => BtKey::NextSection,
            Ok(b'[') => BtKey::PrevSection,
            Ok(d @ b'0'..=b'9') => BtKey::Digit(d),
            Ok(b's') => BtKey::Scan,
            Ok(b'p') => BtKey::Pair,
            Ok(b'f') => BtKey::Forget,
            Ok(b'y') => BtKey::Yes,
            Ok(b'n') => BtKey::No,
            _ => return None,
        },
    };
    Some(key)
}
