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

//! Which usages are keys (HID Usage Tables 1.4): the Keyboard/Keypad page,
//! and the few Consumer controls the Keyboard page also names, so a volume
//! key reaches the session through the same keymap as every other key.

use super::field::Field;
use super::layout::Layout;
use super::span::Dir;

pub const KEYBOARD: u16 = 0x07;
pub const LEDS: u16 = 0x08;
pub const CONSUMER: u16 = 0x0c;

/// From the first letter to Right GUI (HUT 1.4, 10).
const KEYS: (u16, u16) = (0x04, 0xe7);
/// Mute, Volume Increment and Volume Decrement (HUT 1.4, 15), and the
/// Keyboard page's Mute, Volume Up and Volume Down.
const CONSUMER_KEYS: [(u16, u8); 3] = [(0xe2, 0x7f), (0xe9, 0x80), (0xea, 0x81)];

pub fn key_of((page, usage): (u16, u16)) -> Option<u8> {
    match page {
        KEYBOARD if (KEYS.0..=KEYS.1).contains(&usage) => Some(usage as u8),
        CONSUMER => CONSUMER_KEYS.iter().find(|c| c.0 == usage).map(|c| c.1),
        _ => None,
    }
}

/// ErrorRollOver, POSTFail and ErrorUndefined (HUT 1.4, 10): the report
/// cannot say which keys are held.
pub fn phantom((page, usage): (u16, u16)) -> bool {
    page == KEYBOARD && (0x01..=0x03).contains(&usage)
}

/// A field this host reads keys from: an array of usages, or one bit per
/// usage. A wider variable field is a quantity, not a key.
pub fn readable(f: &Field) -> bool {
    f.array || f.size == 1
}

fn gives(f: &Field, typing: bool) -> bool {
    let consumer = |s: &super::span::Span| CONSUMER_KEYS.iter().any(|c| s.hits(CONSUMER, c.0, c.0));
    let page = |s: &super::span::Span| s.hits(KEYBOARD, KEYS.0, KEYS.1);
    readable(f) && f.spans.iter().any(|s| page(s) || (!typing && consumer(s)))
}

/// Whether input report `id` can hold any key.
pub fn has_keys(layout: &Layout, id: u8) -> bool {
    layout.fields(Dir::Input, id).any(|f| gives(f, false))
}

/// Whether input report `id` can hold keys of the Keyboard page itself.
pub fn types(layout: &Layout, id: u8) -> bool {
    layout.fields(Dir::Input, id).any(|f| gives(f, true))
}
