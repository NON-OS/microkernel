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

//! What a key does on the Wi-Fi page, which keeps its own keys: arrows walk
//! the network and saved lists rather than the settings rows. The page's
//! card notes name each letter, so this table and those notes are one list.
//! The Wi-Fi switch had no key at all before: only a click reached it.

use nonos_app_skeleton::{KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_TAB, KEY_UP};

const KEY_SPACE: u32 = 0x20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WifiKey {
    Close,
    NextSection,
    PrevSection,
    Up,
    Down,
    Scan,
    Join,
    Leave,
    Remember,
    Forget,
    /// The Wi-Fi switch, on or off.
    Radio,
}

pub fn wifi_key(code: u32) -> Option<WifiKey> {
    let key = match code {
        KEY_ESC => WifiKey::Close,
        KEY_TAB => WifiKey::NextSection,
        KEY_UP => WifiKey::Up,
        KEY_DOWN => WifiKey::Down,
        // Enter (or Space) always scans, matching the card's "Enter scans".
        KEY_ENTER | KEY_SPACE => WifiKey::Scan,
        _ => match u8::try_from(code).map(|c| c.to_ascii_lowercase()) {
            Ok(b']') => WifiKey::NextSection,
            Ok(b'[') => WifiKey::PrevSection,
            // A key of its own joins, so scanning and joining never fight
            // over the same key.
            Ok(b'c') => WifiKey::Join,
            Ok(b'd') => WifiKey::Leave,
            Ok(b'r') => WifiKey::Remember,
            Ok(b'f') => WifiKey::Forget,
            Ok(b'w') => WifiKey::Radio,
            _ => return None,
        },
    };
    Some(key)
}
