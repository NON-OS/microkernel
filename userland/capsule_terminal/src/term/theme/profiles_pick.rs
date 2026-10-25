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

//! Finding a profile by its number, or by a name typed for it.

use super::{ABYSS, DARK, DIM, LIGHT};
use super::super::types::Theme;

pub fn by_index(i: u16) -> &'static Theme {
    match i {
        1 => &DIM,
        2 => &LIGHT,
        3 => &ABYSS,
        _ => &DARK,
    }
}

pub fn by_name(n: &[u8]) -> Option<u16> {
    match n {
        b"dark" | b"blackarch" | b"black" | b"blue" | b"matrix" => Some(0),
        b"dim" | b"glass" | b"smoke" | b"clear" => Some(1),
        b"light" => Some(2),
        b"abyss" => Some(3),
        _ => None,
    }
}
