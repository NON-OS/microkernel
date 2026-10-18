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

//! What one key event posts, decided apart from the posting so the decision
//! runs on the host: up to two events (a stale code's release first, then
//! this one), each with the code it carries and its flags. The flags hold
//! the modifiers in bits 0 to 6 and, above them, whatever the caller adds:
//! a Bluetooth keyboard marks its source and bond there for the router.
//! What the caller adds never reaches the modifier bits.

use nonos_keymap::HeldKeys;

use super::key_event::KeyEvent;

/// The modifier bits of an input event's flags, which only the keyboard's
/// own state may set.
pub const MODIFIER_BITS: u16 = 0x7F;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Post {
    pub down: bool,
    pub flags: u16,
    pub code: u32,
}

/// The posts for `ev`, whose code resolved to `code`, with `extra` added
/// to their flags above the modifier bits.
pub fn key_posts(ev: KeyEvent, held: &mut HeldKeys, code: u32, extra: u16) -> [Option<Post>; 2] {
    // A release carries the code its press went down with. Resolved again it
    // differed for every Enter, Escape, Backspace and Tab (a release has no
    // ASCII byte, so it posted the raw usage) and for a letter let go after
    // Shift: the router could not pair it with its press, and a Wayland
    // client held the key, repeating it, for good.
    let posts = held.event(u32::from(ev.scancode), !ev.pressed, code);
    let flags = modifier_flags(ev.modifiers, ev.caps) | (extra & !MODIFIER_BITS);
    let first = posts.release_first.map(|code| Post { down: false, flags, code });
    [first, posts.code.map(|code| Post { down: ev.pressed, flags, code })]
}

fn modifier_flags(modifiers: u8, caps: bool) -> u16 {
    let shift = u16::from((modifiers & 0x22) != 0);
    let ctrl = u16::from((modifiers & 0x11) != 0) << 1;
    let alt = u16::from((modifiers & 0x04) != 0) << 2;
    let altgr = u16::from((modifiers & 0x40) != 0) << 6;
    let meta = u16::from((modifiers & 0x88) != 0) << 3;
    let caps = u16::from(caps) << 4;
    shift | ctrl | alt | altgr | meta | caps
}
