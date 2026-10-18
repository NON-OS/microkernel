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

//! Hex literals turned into bytes when the crate builds, so a mistyped
//! constant is a compile error rather than a wrong address at runtime.

const fn nibble(c: u8) -> u8 {
    let v = match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 16,
    };
    assert!(v < 16, "not a hex digit");
    v
}

const fn decode<const N: usize>(s: &str) -> [u8; N] {
    let b = s.as_bytes();
    assert!(b.len() == 2 * N, "wrong number of hex digits");
    let mut out = [0u8; N];
    let mut i = 0;
    while i < N {
        out[i] = (nibble(b[2 * i]) << 4) | nibble(b[2 * i + 1]);
        i += 1;
    }
    out
}

/// A 20-byte address from its 40 hex digits, no `0x`.
pub const fn hex20(s: &str) -> [u8; 20] {
    decode::<20>(s)
}

/// A 32-byte word from its 64 hex digits, no `0x`.
pub const fn hex32(s: &str) -> [u8; 32] {
    decode::<32>(s)
}
