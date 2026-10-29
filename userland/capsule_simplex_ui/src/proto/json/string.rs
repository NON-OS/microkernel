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

//! JSON strings, read with their escapes and written with them.

use alloc::string::String;

use super::parse::Reader;

fn hex4(r: &mut Reader) -> Option<u32> {
    let digits = core::str::from_utf8(r.bytes.get(r.pos..r.pos + 4)?).ok()?;
    r.pos += 4;
    u32::from_str_radix(digits, 16).ok()
}

/// A \u escape, joining a surrogate pair into one character.
fn unicode(r: &mut Reader) -> Option<char> {
    let hi = hex4(r)?;
    if !(0xD800..0xDC00).contains(&hi) {
        return Some(char::from_u32(hi).unwrap_or('\u{FFFD}'));
    }
    if r.bytes.get(r.pos..r.pos + 2)? != b"\\u" {
        return Some('\u{FFFD}');
    }
    r.pos += 2;
    let lo = hex4(r)?;
    let joined = 0x10000 + ((hi - 0xD800) << 10) + (lo.wrapping_sub(0xDC00) & 0x3FF);
    Some(char::from_u32(joined).unwrap_or('\u{FFFD}'))
}

/// The string at the reader, which stands on its opening quote.
pub fn string(r: &mut Reader) -> Option<String> {
    (r.bytes.get(r.pos) == Some(&b'"')).then_some(())?;
    r.pos += 1;
    let mut out = String::new();
    loop {
        let start = r.pos;
        while !matches!(r.bytes.get(r.pos), Some(b'"' | b'\\') | None) {
            r.pos += 1;
        }
        out.push_str(core::str::from_utf8(&r.bytes[start..r.pos]).ok()?);
        let b = *r.bytes.get(r.pos)?;
        r.pos += 1;
        if b == b'"' {
            return Some(out);
        }
        let e = *r.bytes.get(r.pos)?;
        r.pos += 1;
        out.push(match e {
            b'n' => '\n',
            b't' => '\t',
            b'r' => '\r',
            b'b' => '\u{8}',
            b'f' => '\u{C}',
            b'u' => unicode(r)?,
            _ => e as char,
        });
    }
}
