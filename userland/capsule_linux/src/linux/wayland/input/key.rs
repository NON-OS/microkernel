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

//! A key event, written to the client: the Linux key it came from
//! (evdev.rs), then the modifiers it was pressed under (xkb_mods.rs), which
//! is what a client feeds its xkb state with.

use crate::linux::guest::Guest;

use super::super::evdev::evdev_of;
use super::super::out::Event;
use super::super::xkb_mods::{xkb_masks, ALT, ALTGR, CAPS, CTRL, META, NUM, SHIFT};
use super::enter::keyboard_on;

/// wl_keyboard: key is 3, modifiers 4.
const KEYBOARD_KEY: u16 = 3;
const KEYBOARD_MODIFIERS: u16 = 4;

// The bits are the input event's own; xkb_mods.rs names them for the proofs.
const _: () = {
    use nonos_app_skeleton::input::*;
    assert!(SHIFT == MOD_SHIFT && CTRL == MOD_CTRL && ALT == MOD_ALT && META == MOD_META);
    assert!(CAPS == MOD_CAPS && NUM == MOD_NUM && ALTGR == MOD_ALTGR);
};

pub fn key(guest: &mut Guest, code: u32, flags: u16, state: u32) {
    let Some(id) = guest.scene.keyboard else { return };
    let Some(linux) = evdev_of(code) else { return };
    if !keyboard_on(guest, id) {
        return;
    }
    let held = &mut guest.scene.held;
    if !(if state == 1 { held.press(linux) } else { held.release(linux) }) {
        return;
    }
    let serial = guest.scene.next_serial();
    if state == 1 {
        guest.scene.popups.last_press = Some(serial);
    }
    let time = time_ms();
    Event::new(id, KEYBOARD_KEY)
        .u32(serial)
        .u32(time)
        .u32(linux)
        .u32(state)
        .send(&mut guest.display.to_client);
    let (depressed, locked) = xkb_masks(flags);
    let serial = guest.scene.next_serial();
    Event::new(id, KEYBOARD_MODIFIERS)
        .u32(serial)
        .u32(depressed)
        .u32(0)
        .u32(locked)
        .u32(0)
        .send(&mut guest.display.to_client);
}

fn time_ms() -> u32 {
    crate::linux::call::family_ms() as u32
}
