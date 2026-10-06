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

/*
 * The name the Terminal shows as name@host: lowercase letters, digits, -
 * and _, 1 to 32 bytes, a letter first. Every start of a name these rules
 * take is one they take too, so Backspace never leaves a name they refuse.
 * Left empty, the system name stands.
 */

pub const NAME_MAX: usize = 32;
pub const SYSTEM_NAME: &[u8] = b"nonos";

#[derive(Clone, Copy)]
pub enum Refused {
    Capital,
    Space,
    Symbol,
    NotLetterFirst,
    Full,
}

pub fn check(sofar: &[u8], c: u8) -> Result<(), Refused> {
    if sofar.len() >= NAME_MAX {
        return Err(Refused::Full);
    }
    match c {
        b'a'..=b'z' => Ok(()),
        b'A'..=b'Z' => Err(Refused::Capital),
        b' ' => Err(Refused::Space),
        b'0'..=b'9' | b'-' | b'_' if sofar.is_empty() => Err(Refused::NotLetterFirst),
        b'0'..=b'9' | b'-' | b'_' => Ok(()),
        _ => Err(Refused::Symbol),
    }
}

impl Refused {
    /* Why, after the refused key on the step's last line. */
    pub fn text(self) -> &'static [u8] {
        match self {
            Refused::Capital => b"the name is lowercase.",
            Refused::Space => b"no spaces; use - or _ instead.",
            Refused::Symbol => b"only a-z, 0-9, - and _ are taken.",
            Refused::NotLetterFirst => b"the name starts with a letter.",
            Refused::Full => b"32 characters at most.",
        }
    }
}
