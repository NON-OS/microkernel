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

//! The bytes each key sends under the modes a program set.

use alloc::vec::Vec;

use super::keys::{ctrl_byte, Key, Mods};
use super::keys_seq::{letter, tilde};
use crate::term::Modes;

/// Append the bytes for `key` to `out`. Returns false for a key that sends
/// nothing.
pub fn encode_key(key: Key, m: Mods, modes: &Modes, out: &mut Vec<u8>) -> bool {
    let app = modes.cursor_keys;
    match key {
        Key::Char(c) => {
            if m.alt {
                out.push(0x1B);
            }
            match m.ctrl.then(|| ctrl_byte(c)).flatten() {
                Some(b) => out.push(b),
                None => {
                    let mut buf = [0u8; 4];
                    out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                }
            }
        }
        Key::Enter => {
            if m.alt {
                out.push(0x1B);
            }
            out.extend_from_slice(if modes.newline { b"\r\n" } else { b"\r" });
        }
        Key::Tab if m.shift => out.extend_from_slice(b"\x1b[Z"),
        Key::Tab => out.push(b'\t'),
        Key::Backspace => {
            if m.alt {
                out.push(0x1B);
            }
            out.push(if m.ctrl { 0x08 } else { 0x7F });
        }
        Key::Escape => out.push(0x1B),
        Key::Up => letter(out, 'A', m, app),
        Key::Down => letter(out, 'B', m, app),
        Key::Right => letter(out, 'C', m, app),
        Key::Left => letter(out, 'D', m, app),
        Key::Home => letter(out, 'H', m, app),
        Key::End => letter(out, 'F', m, app),
        Key::Insert => tilde(out, 2, m),
        Key::Delete => tilde(out, 3, m),
        Key::PageUp => tilde(out, 5, m),
        Key::PageDown => tilde(out, 6, m),
        Key::F(n @ 1..=4) => letter(out, (b'P' + n - 1) as char, m, true),
        Key::F(n @ 5..=12) => {
            const CODES: [u8; 8] = [15, 17, 18, 19, 20, 21, 23, 24];
            tilde(out, CODES[(n - 5) as usize], m);
        }
        Key::F(_) => return false,
    }
    true
}
