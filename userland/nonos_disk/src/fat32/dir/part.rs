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

//! One part of a short name, base or extension: the character set the
//! format allows, upper-cased into place, and the case it arrived in.

use super::short_name::NameError;

pub const LOWER_BASE: u8 = 0x08;
pub const LOWER_EXT: u8 = 0x10;

pub fn part(text: &str, out: &mut [u8], lower_flag: u8) -> Result<u8, NameError> {
    if text.len() > out.len() {
        return Err(NameError::TooLong);
    }
    let (mut lower, mut upper) = (false, false);
    for (i, &c) in text.as_bytes().iter().enumerate() {
        match c {
            b'a'..=b'z' => lower = true,
            b'A'..=b'Z' => upper = true,
            b'0'..=b'9'
            | b'!'
            | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'('
            | b')'
            | b'-'
            | b'@'
            | b'^'
            | b'_'
            | b'`'
            | b'{'
            | b'}'
            | b'~' => {}
            _ => return Err(NameError::BadChar),
        }
        out[i] = c.to_ascii_uppercase();
    }
    match (lower, upper) {
        (true, true) => Err(NameError::MixedCase),
        (true, false) => Ok(lower_flag),
        _ => Ok(0),
    }
}
