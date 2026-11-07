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

//! What a key does to a passphrase field.

use super::field::Field;

pub const KEY_BACKSPACE: u32 = 0x08;
pub const KEY_ENTER: u32 = 0x0D;
pub const KEY_ESC: u32 = 0x1B;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edit {
    /// The field changed.
    Typed,
    /// Enter.
    Enter,
    /// Escape.
    Escape,
    /// Nothing the field takes.
    Ignored,
}

/// Apply key `code` (a character, or Backspace, Enter, Escape) to `field`.
pub fn edit(field: &mut Field, code: u32) -> Edit {
    match code {
        KEY_ENTER => Edit::Enter,
        KEY_ESC => Edit::Escape,
        KEY_BACKSPACE => {
            field.pop();
            Edit::Typed
        }
        c if (0x20..=0x0010_FFFF).contains(&c) && c != 0x7F => match char::from_u32(c) {
            Some(ch) if field.push(ch) => Edit::Typed,
            _ => Edit::Ignored,
        },
        _ => Edit::Ignored,
    }
}
