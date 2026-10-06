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

use nonos_keymap::HeldKeys;
use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use super::key_event::KeyEvent;
use super::keymap;
use super::post_wire::send;
use super::usage_keycode::usage_keycode;

pub fn publish(ev: KeyEvent, held: &mut HeldKeys) -> bool {
    // A release carries the code its press went down with. Resolved again it
    // differed for every Enter, Escape, Backspace and Tab (a release has no
    // ASCII byte, so it posted the raw usage) and for a letter let go after
    // Shift: the router could not pair it with its press, and a Wayland
    // client held the key, repeating it, for good.
    let posts = held.event(u32::from(ev.scancode), !ev.pressed, key_code(ev));
    let flags = flags(ev.modifiers, ev.caps);
    let mut ok = true;
    if let Some(code) = posts.release_first {
        ok &= send(INPUT_KIND_KEY_UP, flags, code, 0, 0);
    }
    let Some(code) = posts.code else { return ok };
    let kind = if ev.pressed { INPUT_KIND_KEY_DOWN } else { INPUT_KIND_KEY_UP };
    ok & send(kind, flags, code, 0, 0)
}

fn key_code(ev: KeyEvent) -> u32 {
    usage_keycode(ev.scancode).unwrap_or_else(|| resolved_or_usage(ev))
}

// Printable keys post their layout-resolved codepoint (which may be
// outside ASCII for accented letters); control keys keep the ASCII byte
// from the event; anything else posts its raw usage in the 0x2000 page.
fn resolved_or_usage(ev: KeyEvent) -> u32 {
    let code = keymap::resolve_code(ev.scancode, ev.modifiers, ev.caps);
    if code != 0 {
        return code;
    }
    match ev.ascii {
        0 => 0x2000 | ev.scancode as u32,
        b'\n' => 0x0D,
        c => c as u32,
    }
}

fn flags(modifiers: u8, caps: bool) -> u16 {
    let shift = u16::from((modifiers & 0x22) != 0);
    let ctrl = u16::from((modifiers & 0x11) != 0) << 1;
    let alt = u16::from((modifiers & 0x04) != 0) << 2;
    let altgr = u16::from((modifiers & 0x40) != 0) << 6;
    let meta = u16::from((modifiers & 0x88) != 0) << 3;
    let caps = u16::from(caps) << 4;
    shift | ctrl | alt | altgr | meta | caps
}
