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

use super::encoding::Encoding;
use super::label::is_ascii_space;
use super::prescan_attr::{attribute, Attr};
use super::prescan_meta::meta;
use super::prescan_value::{gap, tag_start};
use super::xml_decl::xml_encoding;

/* HTML's "prescan a byte stream to determine its encoding" over the first
`limit` bytes: comments and other tags are stepped over, the first
<meta> that declares a known encoding wins. Running out of bytes ends
in "get an XML encoding" on the same bytes. */
pub fn prescan(b: &[u8], limit: usize) -> Option<Encoding> {
    let b = &b[..b.len().min(limit)];
    if b.starts_with(&[0x3C, 0, 0x3F, 0, 0x78, 0]) {
        return Some(Encoding::Utf16Le);
    }
    if b.starts_with(&[0, 0x3C, 0, 0x3F, 0, 0x78]) {
        return Some(Encoding::Utf16Be);
    }
    let mut i = 0;
    while i < b.len() {
        let rest = &b[i..];
        if rest.starts_with(b"<!--") {
            /* To the '>' of the first "-->", whose dashes may be those of "<!--". */
            let Some(end) = b[i + 2..].windows(3).position(|w| w == b"-->") else { break };
            i += 2 + end + 2;
        } else if rest.len() > 5 && rest[..5].eq_ignore_ascii_case(b"<meta") && gap(rest[5]) {
            i += 5;
            match meta(b, &mut i) {
                Attr::Found(e) => return Some(e),
                Attr::Out => break,
                Attr::None => {}
            }
        } else if tag_start(rest) {
            let Some(gap) = rest.iter().position(|&c| is_ascii_space(c) || c == b'>') else {
                break;
            };
            i += gap;
            loop {
                match attribute(b, &mut i) {
                    Attr::Found(_) => continue,
                    Attr::None => break,
                    Attr::Out => return xml_encoding(b),
                }
            }
        } else if rest.starts_with(b"<!") || rest.starts_with(b"</") || rest.starts_with(b"<?") {
            let Some(end) = rest[1..].iter().position(|&c| c == b'>') else { break };
            i += 1 + end;
        }
        i += 1;
    }
    xml_encoding(b)
}
