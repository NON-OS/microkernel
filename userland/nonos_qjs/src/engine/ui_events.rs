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

//! Keys and pointer presses sent to the page with what they carry.

use super::ffi::{njs_dispatch_key, njs_dispatch_mouse};
use super::lifecycle::Engine;

/// Modifier bits an event carries: `shiftKey`, `ctrlKey`, `altKey` and
/// `metaKey` read them.
pub const MOD_SHIFT: u8 = 1;
pub const MOD_CTRL: u8 = 2;
pub const MOD_ALT: u8 = 4;
pub const MOD_META: u8 = 8;

/// A key as a page reads it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key<'a> {
    /// What the key means: "a", "A", "Enter", "ArrowUp", " ".
    pub key: &'a str,
    /// The key it is on: "KeyA", "Enter", "Space"; empty when not known.
    pub code: &'a str,
    /// The legacy `keyCode` and `which`.
    pub key_code: i32,
    /// `MOD_*` bits.
    pub mods: u8,
}

/// A pointer press as a page reads it, in the page area's coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Press {
    pub x: i32,
    pub y: i32,
    /// 0 main, 1 middle, 2 second.
    pub button: i32,
    /// The buttons held after this event, as bits: 1 main, 2 second, 4 middle.
    pub buttons: i32,
    /// `MOD_*` bits.
    pub mods: u8,
}

/* A NUL-terminated copy of `s`, cut to fit `N` bytes on a character. */
fn cstr<const N: usize>(s: &str) -> [u8; N] {
    let mut buf = [0u8; N];
    let mut n = s.len().min(N - 1);
    while n > 0 && !s.is_char_boundary(n) {
        n -= 1;
    }
    buf[..n].copy_from_slice(&s.as_bytes()[..n]);
    buf
}

impl Engine {
    /// Send `ty` (keydown, keypress or keyup) to `node` and up through its
    /// ancestors, the document and window, carrying `key`. Answers how many
    /// listeners ran; `default_prevented` then says whether one cancelled
    /// it, and the browser leaves the key alone if so.
    pub fn dispatch_key(&self, node: i32, ty: &str, key: &Key) -> i32 {
        let (t, k, c) = (cstr::<32>(ty), cstr::<32>(key.key), cstr::<32>(key.code));
        unsafe {
            njs_dispatch_key(
                self.ctx,
                node,
                t.as_ptr(),
                k.as_ptr(),
                c.as_ptr(),
                key.key_code,
                key.mods as i32,
            )
        }
    }

    /// Send `ty` (mousedown, mouseup, click, pointerdown or pointerup) to
    /// `node` and up, carrying where the pointer was and which button.
    pub fn dispatch_press(&self, node: i32, ty: &str, p: &Press) -> i32 {
        let t = cstr::<32>(ty);
        unsafe {
            njs_dispatch_mouse(
                self.ctx,
                node,
                t.as_ptr(),
                p.x,
                p.y,
                p.button,
                p.buttons,
                p.mods as i32,
            )
        }
    }
}
