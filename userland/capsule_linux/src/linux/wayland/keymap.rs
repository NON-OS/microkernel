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

//! `wl_keyboard.keymap`.

use crate::linux::file::install;
use crate::linux::guest::{Fd, Guest};

use super::keymap_file::load;
use super::out::Event;

const KEYBOARD_KEYMAP: u16 = 0;
const KEYBOARD_REPEAT_INFO: u16 = 5;
/// The client repeats a held key itself, 25 a second after 600 ms, the
/// rate and delay most desktops give (held_keys.rs says why).
const REPEAT_RATE: u32 = 25;
const REPEAT_DELAY_MS: u32 = 600;
const FORMAT_NO_KEYMAP: u32 = 0;
const FORMAT_XKB_V1: u32 = 1;

pub fn send(guest: &mut Guest, keyboard: u32) {
    let text = load();
    let (format, size) = match &text {
        Some(bytes) => (FORMAT_XKB_V1, bytes.len() as u32 + 1),
        None => (FORMAT_NO_KEYMAP, 1),
    };
    let Some(fd) = install(guest, Fd::memfd()) else {
        return;
    };
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        entry.size = size as u64;
        if let Some(bytes) = text {
            entry.pending = bytes;
            entry.pending.push(0);
        }
    }
    guest.display.give.push(fd as u32);
    Event::new(keyboard, KEYBOARD_KEYMAP).u32(format).u32(size).send(&mut guest.display.to_client);
    // repeat_info came with wl_keyboard 4; an older one repeats as it likes.
    if guest.objects.at_least(keyboard, 4) {
        Event::new(keyboard, KEYBOARD_REPEAT_INFO)
            .u32(REPEAT_RATE)
            .u32(REPEAT_DELAY_MS)
            .send(&mut guest.display.to_client);
    }
}
