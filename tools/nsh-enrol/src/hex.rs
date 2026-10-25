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

//! Hex in and out, for the values an operator types or reads.

/// Exactly `N` bytes from `2 * N` hex digits, either case.
pub fn array<const N: usize>(s: &str, what: &'static str) -> Result<[u8; N], crate::Error> {
    if s.len() != 2 * N || !s.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(crate::Error::Value(what));
    }
    let mut out = [0u8; N];
    for (i, b) in out.iter_mut().enumerate() {
        let pair = s.get(2 * i..2 * i + 2).ok_or(crate::Error::Value(what))?;
        *b = u8::from_str_radix(pair, 16).map_err(|_| crate::Error::Value(what))?;
    }
    Ok(out)
}

pub fn of(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
