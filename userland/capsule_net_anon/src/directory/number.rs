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

//! Reading decimal numbers and dotted addresses out of a document.

/// Parse a decimal number. `None` on an empty field, a non-digit, or a value
/// past `u64`, so a document can never make this wrap or panic.
pub fn decimal(text: &[u8]) -> Option<u64> {
    if text.is_empty() {
        return None;
    }
    let mut value = 0u64;
    for byte in text {
        if !byte.is_ascii_digit() {
            return None;
        }
        value = value.checked_mul(10)?.checked_add((byte - b'0') as u64)?;
    }
    Some(value)
}

/// Parse a port. `None` for anything that will not fit, zero included: a relay
/// advertising port zero is not reachable and is not worth carrying.
pub fn port(text: &[u8]) -> Option<u16> {
    match decimal(text)? {
        0 => None,
        value if value <= u16::MAX as u64 => Some(value as u16),
        _ => None,
    }
}

/// Parse a dotted quad IPv4 address.
///
pub fn ipv4(text: &[u8]) -> Option<[u8; 4]> {
    let mut out = [0u8; 4];
    let mut parts = text.split(|b| *b == b'.');
    for slot in out.iter_mut() {
        let value = decimal(parts.next()?)?;
        if value > 255 {
            return None;
        }
        *slot = value as u8;
    }
    if parts.next().is_some() {
        return None;
    }
    Some(out)
}
