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

use super::meta_content::from_content;
use super::prescan_attr::{attribute, Attr};
use super::{encoding, Encoding, WINDOWS_1252};

/* The attributes of one <meta> during the prescan. `charset="..."`
declares an encoding; `content="...; charset=..."` does only beside
`http-equiv="content-type"`. A repeated attribute is ignored. UTF-16
is taken as UTF-8 and x-user-defined as windows-1252, since the page
being read in ASCII is neither. */
pub fn meta(b: &[u8], i: &mut usize) -> Attr<Encoding> {
    let mut seen: Vec<Vec<u8>> = Vec::new();
    let (mut got_pragma, mut need_pragma) = (false, None);
    let mut charset: Option<Option<Encoding>> = None;
    loop {
        let (name, value) = match attribute(b, i) {
            Attr::Found(pair) => pair,
            Attr::None => break,
            Attr::Out => return Attr::Out,
        };
        if seen.contains(&name) {
            continue;
        }
        match &name[..] {
            b"http-equiv" => got_pragma |= value == b"content-type",
            b"content" if charset.is_none() => {
                if let Some(e) = from_content(&value) {
                    (charset, need_pragma) = (Some(Some(e)), Some(true));
                }
            }
            b"charset" => (charset, need_pragma) = (Some(encoding(&value)), Some(false)),
            _ => {}
        }
        seen.push(name);
    }
    match (need_pragma, charset) {
        (Some(need), Some(Some(e))) if got_pragma || !need => Attr::Found(match e {
            Encoding::XUserDefined => WINDOWS_1252,
            e => e.ascii_compatible(),
        }),
        _ => Attr::None,
    }
}
