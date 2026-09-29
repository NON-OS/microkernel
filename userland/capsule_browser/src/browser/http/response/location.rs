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

use alloc::string::String;

/* A Location value as text. Its bytes are read as UTF-8, and a byte that
is not part of valid UTF-8 (obs-text such as a raw Latin-1 letter) is
percent-encoded, as browsers do, so the redirect target keeps it. */
pub fn location(v: &[u8]) -> String {
    let mut s = String::with_capacity(v.len());
    for chunk in v.utf8_chunks() {
        s.push_str(chunk.valid());
        for &b in chunk.invalid() {
            s.push('%');
            s.push(char::from(b"0123456789ABCDEF"[usize::from(b >> 4)]));
            s.push(char::from(b"0123456789ABCDEF"[usize::from(b & 15)]));
        }
    }
    s
}
