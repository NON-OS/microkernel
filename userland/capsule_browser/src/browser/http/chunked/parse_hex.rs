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

/* The chunk size of a size line: hex digits, then optional whitespace
(the BWS RFC 9112 7.1.1 allows before a chunk extension, and the
padding some servers add), then any extension. A size that does not
fit a signed 64-bit count is refused, as browsers do. */
pub fn parse_hex(b: &[u8]) -> Option<usize> {
    let head = b.split(|&c| c == b';').next()?;
    let digits = head.len() - head.iter().rev().take_while(|&&c| c == b' ' || c == b'\t').count();
    let head = &head[..digits];
    if head.is_empty() {
        return None;
    }
    let mut v = 0u64;
    for &c in head {
        let d = match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => return None,
        };
        v = v.checked_mul(16)?.checked_add(u64::from(d))?;
    }
    if v > i64::MAX as u64 {
        return None;
    }
    usize::try_from(v).ok()
}
