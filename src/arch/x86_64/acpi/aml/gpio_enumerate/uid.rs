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

use super::super::scan::find_name_value;

/// Decode a `Name (_UID, value)` in the device body as a small integer: the
/// constant opcodes Zero/One, the Byte/Word/DWord const prefixes, or a decimal
/// string (some firmware declares `_UID` as `"1"`). None when absent or in an
/// encoding we do not model; the caller then defaults to community zero.
pub(super) fn parse_uid(body: &[u8]) -> Option<u32> {
    let at = find_name_value(body, b"_UID")?;
    let v = body.get(at..)?;
    match v.first().copied()? {
        0x00 => Some(0),
        0x01 => Some(1),
        0x0A => Some(u32::from(*v.get(1)?)),
        0x0B => Some(u32::from(u16::from_le_bytes([*v.get(1)?, *v.get(2)?]))),
        0x0C => Some(u32::from_le_bytes([*v.get(1)?, *v.get(2)?, *v.get(3)?, *v.get(4)?])),
        0x0D => parse_decimal_string(v.get(1..)?),
        _ => None,
    }
}

/// Parse a NUL-terminated ASCII decimal string into a u32, rejecting anything
/// that is not purely digits or that overflows.
fn parse_decimal_string(bytes: &[u8]) -> Option<u32> {
    let mut value: u32 = 0;
    let mut digits = 0usize;
    for &b in bytes {
        if b == 0 {
            break;
        }
        if !b.is_ascii_digit() {
            return None;
        }
        value = value.checked_mul(10)?.checked_add(u32::from(b - b'0'))?;
        digits += 1;
    }
    (digits > 0).then_some(value)
}
