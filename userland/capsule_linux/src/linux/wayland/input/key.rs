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

use super::super::evdev::{evdev_of, us_shift};
use super::super::out::Event;
use super::super::xkb_mods::{
    as_us, is_modifier, key_say, xkb_masks, ALT, ALTGR, CAPS, CTRL, META, NUM, SHIFT,
};
use super::devices::keyboards;
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
    let Some(linux) = evdev_of(code) else { return };
    let keyboards = keyboards(guest);
    if keyboards.is_empty() || !keyboard_on(guest) {
        return;
    }
    let held = &mut guest.scene.held;
    // A driver's repeat of a key held down: let go and pressed again, but
    // for a modifier, which is held, not typed.
    if state == 1 && held.is_down(linux) {
        if !is_modifier(linux) {
            repeat(guest, &keyboards, linux, (xkb_masks(flags), us_shift(code)));
        }
        return;
    }
    let news = if state == 1 { held.press(linux) } else { held.release(linux) };
    /* Ctrl+Q is Ctrl only if Ctrl was said before the Q (xkb_mods::after). */
    let told = xkb_masks(flags);
    let typed = if state == 1 { as_us(us_shift(code), told) } else { told };
    let say = key_say(linux, state == 1, news, (told, typed));
    if let Some(masks) = say.before {
        mods(guest, &keyboards, masks);
    }
    if !say.key {
        if let Some(masks) = say.after {
            mods(guest, &keyboards, masks);
        }
        return;
    }
    let serial = guest.scene.next_serial();
    match state {
        1 => guest.scene.popups.pressed(serial),
        // GTK takes a menu's grab with the latest key serial it was sent,
        // and a key let go of before the menu maps is that serial.
        _ => guest.scene.popups.last_key_up = Some(serial),
    }
    let time = crate::linux::call::family_ms() as u32;
    for &id in &keyboards {
        let ev = Event::new(id, KEYBOARD_KEY).u32(serial).u32(time).u32(linux).u32(state);
        ev.send(&mut guest.display.to_client);
    }
    if let Some(masks) = say.after {
        mods(guest, &keyboards, masks);
    }
}

/// A held key's repeat to every keyboard told not to repeat keys itself
/// (wl_keyboard 4 on, keymap.rs): a release and a press, as one more key
/// typed. A keyboard older than that repeats as it likes, and is told none.
fn repeat(
    guest: &mut Guest,
    keyboards: &[u32],
    linux: u32,
    (told, shift): ((u32, u32), Option<bool>),
) {
    mods(guest, keyboards, as_us(shift, told));
    let time = crate::linux::call::family_ms() as u32;
    for state in [0, 1] {
        let serial = guest.scene.next_serial();
        match state {
            1 => guest.scene.popups.pressed(serial),
            _ => guest.scene.popups.last_key_up = Some(serial),
        }
        for &id in keyboards.iter().filter(|&&id| guest.objects.at_least(id, 4)) {
            let ev = Event::new(id, KEYBOARD_KEY).u32(serial).u32(time).u32(linux).u32(state);
            ev.send(&mut guest.display.to_client);
        }
    }
    mods(guest, keyboards, told);
}

/// wl_keyboard.modifiers with `masks`, when the client does not have them.
fn mods(guest: &mut Guest, keyboards: &[u32], (depressed, locked): (u32, u32)) {
    if !guest.scene.held.mods((depressed, locked)) {
        return;
    }
    let serial = guest.scene.next_serial();
    for &id in keyboards {
        let ev = Event::new(id, KEYBOARD_MODIFIERS).u32(serial).u32(depressed).u32(0);
        ev.u32(locked).u32(0).send(&mut guest.display.to_client);
    }
}
