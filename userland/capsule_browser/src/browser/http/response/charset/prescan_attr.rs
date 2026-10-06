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

/* A step of the prescan: something found, nothing there, or the bytes
ran out (which ends the prescan). */
pub enum Attr<T> {
    Found(T),
    None,
    Out,
}

/* The prescan's "get an attribute" at `*i`: a lowercase name and value
(quoted, bare or empty), `None` at the '>' that ends the tag. */
pub fn attribute(b: &[u8], i: &mut usize) -> Attr<(Vec<u8>, Vec<u8>)> {
    let at = |i: usize| b.get(i).copied();
    while at(*i).is_some_and(|c| is_ascii_space(c) || c == b'/') {
        *i += 1;
    }
    match at(*i) {
        None => return Attr::Out,
        Some(b'>') => return Attr::None,
        Some(_) => {}
    }
    let (mut name, value) = (Vec::new(), Vec::new());
    loop {
        let Some(c) = at(*i) else { return Attr::Out };
        if c == b'=' && !name.is_empty() {
            *i += 1;
            break;
        }
        if is_ascii_space(c) {
            while at(*i).is_some_and(is_ascii_space) {
                *i += 1;
            }
            match at(*i) {
                None => return Attr::Out,
                Some(b'=') => *i += 1,
                Some(_) => return Attr::Found((name, value)),
            }
            break;
        }
        if c == b'/' || c == b'>' {
            return Attr::Found((name, value));
        }
        name.push(c.to_ascii_lowercase());
        *i += 1;
    }
    super::prescan_value::value(b, i, name)
}
