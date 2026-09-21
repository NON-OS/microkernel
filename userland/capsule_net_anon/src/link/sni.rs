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

//! The server name sent in the ClientHello.

/// Write `address` as dotted quad into `scratch` and return it as a str.
///
pub fn write(scratch: &mut [u8; 15], address: [u8; 4]) -> &[u8] {
    let mut at = 0usize;
    for (index, octet) in address.iter().enumerate() {
        if index > 0 && at < scratch.len() {
            scratch[at] = b'.';
            at += 1;
        }
        at += decimal(&mut scratch[at..], *octet);
    }
    &scratch[..at]
}

fn decimal(out: &mut [u8], value: u8) -> usize {
    let mut digits = [0u8; 3];
    let mut count = 0usize;
    let mut left = value;
    loop {
        digits[count] = b'0' + left % 10;
        left /= 10;
        count += 1;
        if left == 0 {
            break;
        }
    }
    for index in 0..count {
        if index < out.len() {
            out[index] = digits[count - 1 - index];
        }
    }
    count.min(out.len())
}
