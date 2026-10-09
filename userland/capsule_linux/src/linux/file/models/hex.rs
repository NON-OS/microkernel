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

/*
 * A 64-digit hex digest as its 32 bytes, at compile time. A digit that is not
 * lowercase hex gives the all-zero digest, which no file hashes to, so a
 * mistyped pin admits nothing; model_fetch_proofs holds that no pin is zero.
 */
pub const fn hex32(h: &[u8; 64]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 64 {
        let n = match h[i] {
            b'0'..=b'9' => h[i] - b'0',
            b'a'..=b'f' => h[i] - b'a' + 10,
            _ => return [0; 32],
        };
        out[i / 2] |= n << (4 * (1 - i % 2));
        i += 1;
    }
    out
}
