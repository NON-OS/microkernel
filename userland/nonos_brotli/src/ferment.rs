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

/// The uppercasing of RFC 7932 appendix B for the character at `pos`:
/// ASCII letters flip case, longer UTF-8 sequences flip a fixed bit.
pub(crate) fn ferment(w: &mut [u8], pos: usize) -> usize {
    let step = match w[pos] {
        b'a'..=b'z' => {
            w[pos] ^= 32;
            1
        }
        0..=191 => 1,
        192..=223 => 2,
        _ => 3,
    };
    match step {
        2 if pos + 1 < w.len() => w[pos + 1] ^= 32,
        3 if pos + 2 < w.len() => w[pos + 2] ^= 5,
        _ => {}
    }
    step
}
