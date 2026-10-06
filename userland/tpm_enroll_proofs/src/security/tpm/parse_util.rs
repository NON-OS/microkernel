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

//! Bytes for the parse tests: hex, and a response around a body.

pub(super) fn unhex(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex")).collect()
}

/// A response with code `rc`, then `body`.
pub(super) fn resp(rc: u32, body: &[u8]) -> Vec<u8> {
    let mut r = vec![0x80, 0x01];
    r.extend_from_slice(&((10 + body.len()) as u32).to_be_bytes());
    r.extend_from_slice(&rc.to_be_bytes());
    r.extend_from_slice(body);
    r
}
