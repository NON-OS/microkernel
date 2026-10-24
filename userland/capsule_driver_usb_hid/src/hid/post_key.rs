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
use super::key_post::key_posts;
use super::keymap;
use super::post_wire::send;
use super::usage_keycode::usage_keycode;

/// Post `ev`, with `extra` added to its flags above the modifier bits: 0
/// from a wired keyboard, which posts what it always has.
pub fn publish(ev: KeyEvent, held: &mut HeldKeys, extra: u16) -> bool {
    let mut ok = true;
    for p in key_posts(ev, held, key_code(ev), extra).into_iter().flatten() {
        let kind = if p.down { INPUT_KIND_KEY_DOWN } else { INPUT_KIND_KEY_UP };
        ok &= send(kind, p.flags, p.code, 0, 0);
    }
    ok
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
