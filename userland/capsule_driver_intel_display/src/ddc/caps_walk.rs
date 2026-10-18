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

//! Walking a vcp list: each code, then the values in parentheses after a
//! code, if any, as pairs of hex digits with any spacing.

/// Each code (`value` None), and each value listed after a code (`Some`),
/// in order. None for a byte that is neither hex, space nor parenthesis,
/// an odd digit, or values nested deeper than one level.
pub fn each(list: &[u8], mut f: impl FnMut(u8, Option<u8>)) -> Option<()> {
    let (mut code, mut depth, mut digit) = (None, 0u8, None::<u8>);
    for &b in list {
        match (b, digit) {
            (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F', None) => digit = Some(hex(b)),
            (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F', Some(h)) => {
                let v = h << 4 | hex(b);
                match depth {
                    0 => {
                        code = Some(v);
                        f(v, None)
                    }
                    _ => f(code?, Some(v)),
                }
                digit = None;
            }
            (b' ' | b'\t' | b'\r' | b'\n', None) => {}
            (b'(', None) if depth == 0 && code.is_some() => depth = 1,
            (b')', None) if depth == 1 => depth = 0,
            _ => return None,
        }
    }
    (depth == 0 && digit.is_none()).then_some(())
}

fn hex(b: u8) -> u8 {
    (b as char).to_digit(16).unwrap_or(0) as u8
}
