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
use super::label::encoding;

/* HTML's "get an XML encoding": the quoted `encoding` value of an XML
declaration at the very start (`<?xml ... encoding="..." ...>`),
UTF-16 taken as UTF-8. */
pub fn xml_encoding(b: &[u8]) -> Option<Encoding> {
    if !b.starts_with(b"<?xml") {
        return None;
    }
    let end = b.iter().position(|&c| c == b'>')?;
    let mut p = b[..end].windows(8).position(|w| w == b"encoding")? + 8;
    let skip = |p: &mut usize| {
        while b.get(*p).is_some_and(|&c| c <= 0x20) {
            *p += 1;
        }
    };
    skip(&mut p);
    if b.get(p) != Some(&b'=') {
        return None;
    }
    p += 1;
    skip(&mut p);
    let &quote = b.get(p).filter(|&&q| q == b'"' || q == b'\'')?;
    p += 1;
    let len = b[p..].iter().position(|&c| c == quote)?;
    let name = &b[p..p + len];
    if name.iter().any(|&c| c <= 0x20) {
        return None;
    }
    encoding(name).map(Encoding::ascii_compatible)
}
