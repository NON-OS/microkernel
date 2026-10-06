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

use alloc::vec::Vec;

use super::label::is_ascii_space;
use super::prescan_attr::Attr;

/* The value half of "get an attribute", after the '=': whitespace, then
a quoted value up to the same quote, an empty value at '>', or a bare
value up to whitespace or '>'. Letters are lowercased. */
pub fn value(b: &[u8], i: &mut usize, name: Vec<u8>) -> Attr<(Vec<u8>, Vec<u8>)> {
    let at = |i: usize| b.get(i).copied();
    while at(*i).is_some_and(is_ascii_space) {
        *i += 1;
    }
    let mut value = Vec::new();
    match at(*i) {
        None => return Attr::Out,
        Some(q @ (b'"' | b'\'')) => loop {
            *i += 1;
            match at(*i) {
                None => return Attr::Out,
                Some(c) if c == q => {
                    *i += 1;
                    return Attr::Found((name, value));
                }
                Some(c) => value.push(c.to_ascii_lowercase()),
            }
        },
        Some(b'>') => return Attr::Found((name, value)),
        Some(c) => {
            value.push(c.to_ascii_lowercase());
            *i += 1;
        }
    }
    loop {
        match at(*i) {
            None => return Attr::Out,
            Some(c) if is_ascii_space(c) || c == b'>' => return Attr::Found((name, value)),
            Some(c) => value.push(c.to_ascii_lowercase()),
        }
        *i += 1;
    }
}

/* What may follow "<meta": whitespace or '/'. */
pub fn gap(c: u8) -> bool {
    is_ascii_space(c) || c == b'/'
}

/* '<', an optional '/', then an ASCII letter. */
pub fn tag_start(r: &[u8]) -> bool {
    let letter = |k: usize| r.get(k).is_some_and(u8::is_ascii_alphabetic);
    r.first() == Some(&b'<') && (letter(1) || (r.get(1) == Some(&b'/') && letter(2)))
}
