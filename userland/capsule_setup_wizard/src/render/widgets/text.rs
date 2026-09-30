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
 * Short texts put together in a caller's buffer, cut at its end.
 */

pub fn cat<'a>(out: &'a mut [u8], parts: &[&[u8]]) -> &'a [u8] {
    let mut len = 0;
    for p in parts {
        let take = p.len().min(out.len() - len);
        out[len..len + take].copy_from_slice(&p[..take]);
        len += take;
    }
    &out[..len]
}

/*
 * A byte count as the Terminal's qwen says it: tenths of a GB from a
 * billion bytes up, else whole MB rounded up.
 */
pub fn size(bytes: u64, out: &mut [u8; 24]) -> &[u8] {
    let (whole, tenth, unit): (u64, Option<u64>, &[u8]) = match bytes {
        b if b >= 1_000_000_000 => (b / 1_000_000_000, Some(b / 100_000_000 % 10), b" GB"),
        b => (b.div_ceil(1_000_000), None, b" MB"),
    };
    let mut digits = [0u8; 20];
    let mut n = digits.len();
    let mut left = whole;
    loop {
        n -= 1;
        digits[n] = b'0' + (left % 10) as u8;
        left /= 10;
        if left == 0 {
            break;
        }
    }
    let point = [b'.', b'0' + tenth.unwrap_or(0) as u8];
    let point: &[u8] = if tenth.is_some() { &point } else { b"" };
    cat(out, &[&digits[n..], point, unit])
}
