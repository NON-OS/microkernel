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

//! Reading a count of seconds out of the command line. Pure, for the host
//! proofs.

/// The first run of decimal digits, saturating; none, or zero, is no number.
pub fn first_number(bytes: &[u8]) -> Option<u64> {
    let start = bytes.iter().position(u8::is_ascii_digit)?;
    let mut value: u64 = 0;
    for &b in bytes[start..].iter().take_while(|b| b.is_ascii_digit()) {
        value = value.saturating_mul(10).saturating_add(u64::from(b - b'0'));
    }
    (value != 0).then_some(value)
}
