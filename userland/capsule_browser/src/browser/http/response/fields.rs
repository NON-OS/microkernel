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

use alloc::borrow::Cow;

use crate::browser::http::chunked::line_end;

use super::bytes::{is_ows, is_tchar, trim};

/* Calls `f` with the name and value of each field line of a header
section (after its status line, up to the blank line). An obs-fold
continuation line is joined to its field with one SP (RFC 9112 5.2);
one before any field, and a line with no colon or a name that is not
a token, is skipped. Space before the colon is tolerated. */
pub fn each_field(fields: &[u8], mut f: impl FnMut(&[u8], &[u8])) {
    let mut cur: Option<(&[u8], Cow<[u8]>)> = None;
    let mut i = 0;
    while let Some((end, next)) = line_end(&fields[i..]) {
        let line = &fields[i..i + end];
        i += next;
        if line.is_empty() {
            break;
        }
        if is_ows(line[0]) {
            if let Some((_, v)) = cur.as_mut() {
                let v = v.to_mut();
                v.push(b' ');
                v.extend_from_slice(trim(line));
            }
            continue;
        }
        if let Some((n, v)) = cur.take() {
            f(n, trim(&v));
        }
        let Some(colon) = line.iter().position(|&c| c == b':') else { continue };
        let name = trim(&line[..colon]);
        if !name.is_empty() && name.iter().all(|&c| is_tchar(c)) {
            cur = Some((name, Cow::Borrowed(&line[colon + 1..])));
        }
    }
    if let Some((n, v)) = cur {
        f(n, trim(&v));
    }
}
