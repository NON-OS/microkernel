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

//! Bytes as 0x hex, the way the fields of a request carry them.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub fn hex(b: &[u8]) -> String {
    let body: String = b.iter().map(|x| format!("{x:02x}")).collect();
    format!("0x{body}")
}

pub fn unhex(s: &str) -> Option<Vec<u8>> {
    let s = s.strip_prefix("0x")?.as_bytes();
    let digit = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let pair = |p: &[u8]| Some(digit(*p.first()?)? << 4 | digit(*p.get(1)?)?);
    (s.len() % 2 == 0).then(|| s.chunks(2).map(pair).collect()).flatten()
}
