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

//! Hex text as the answers carry it.

use alloc::vec::Vec;

/// "0x" and an even count of hex digits, as bytes.
pub fn hex_bytes(text: &[u8]) -> Option<Vec<u8>> {
    let digits = text.strip_prefix(b"0x")?;
    if digits.len() % 2 != 0 {
        return None;
    }
    let nib = |c: u8| (c as char).to_digit(16).map(|v| v as u8);
    digits.chunks(2).map(|p| Some(nib(p[0])? << 4 | nib(p[1])?)).collect()
}

/// A quantity "0x..." as a u64.
pub fn quantity(text: &[u8]) -> Option<u64> {
    let digits = text.strip_prefix(b"0x")?;
    if digits.is_empty() || digits.len() > 16 {
        return None;
    }
    digits.iter().try_fold(0u64, |n, d| Some(n << 4 | (*d as char).to_digit(16)? as u64))
}
