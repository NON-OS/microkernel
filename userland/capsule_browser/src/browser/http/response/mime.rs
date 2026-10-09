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

use super::bytes::{is_tchar, trim};

/* The response's MIME type as Fetch's "extract a MIME type" reads every
Content-Type value: the last parsable type wins, and a charset carries
over from an earlier value of the same essence. */
pub struct Mime {
    /* A Content-Type field was sent at all. */
    pub present: bool,
    /* Lowercase `type/subtype`; empty when no value parsed. */
    pub essence: Vec<u8>,
    pub charset: Option<Vec<u8>>,
}

impl Mime {
    pub const NONE: Mime = Mime { present: false, essence: Vec::new(), charset: None };

    pub fn feed(&mut self, value: &[u8]) {
        self.present = true;
        for item in split_unquoted(value, b',') {
            let mut parts = split_unquoted(item, b';');
            let essence = trim(parts.next().unwrap_or(item)).to_ascii_lowercase();
            let Some(slash) = essence.iter().position(|&c| c == b'/') else { continue };
            let (ty, sub) = (&essence[..slash], &essence[slash + 1..]);
            let token = ty.iter().chain(sub).all(|&c| is_tchar(c));
            if ty.is_empty() || sub.is_empty() || !token || essence == b"*/*" {
                continue;
            }
            let charset = parts.find_map(charset_param);
            if essence != self.essence {
                (self.essence, self.charset) = (essence, charset);
            } else if charset.is_some() {
                self.charset = charset;
            }
        }
    }
}

/* The value of a `charset=` parameter, unquoted. */
fn charset_param(p: &[u8]) -> Option<Vec<u8>> {
    let eq = p.iter().position(|&c| c == b'=')?;
    if !trim(&p[..eq]).eq_ignore_ascii_case(b"charset") {
        return None;
    }
    let v = trim(&p[eq + 1..]);
    let v = v.strip_prefix(b"\"").map_or(v, |q| q.split(|&c| c == b'"').next().unwrap_or(q));
    (!v.is_empty()).then(|| v.to_vec())
}

/* `v` split at `sep` bytes outside double quotes. */
fn split_unquoted(v: &[u8], sep: u8) -> impl Iterator<Item = &[u8]> {
    let mut quoted = false;
    v.split(move |&c| {
        quoted ^= c == b'"';
        c == sep && !quoted
    })
}
