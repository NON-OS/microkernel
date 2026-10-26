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
//! The lines the shell's start is read by on the serial port, beside the
//! kernel's own `[BENCH] <uptime ms> <what>` marks.

/// "[BENCH] 122095 shell_<step>\n" into `out`; its length.
pub fn mark_line(at_ms: i64, step: &[u8], out: &mut [u8; 96]) -> usize {
    let mut n = put(out, 0, b"[BENCH] ");
    n = decimal(out, n, at_ms.max(0) as u64);
    n = put(out, n, b" shell_");
    n = put(out, n, step);
    put(out, n, b"\n")
}

/// "[SHELL] answered op 0x0007 at 125000 ms, 2905 ms after the shell began\n".
pub fn answered_line(op: u16, at_ms: i64, since_ms: i64, out: &mut [u8; 96]) -> usize {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut n = put(out, 0, b"[SHELL] answered op 0x");
    for shift in [12, 8, 4, 0] {
        n = put(out, n, &[HEX[(op >> shift) as usize & 0xF]]);
    }
    n = put(out, n, b" at ");
    n = decimal(out, n, at_ms.max(0) as u64);
    n = put(out, n, b" ms, ");
    n = decimal(out, n, since_ms.max(0) as u64);
    put(out, n, b" ms after the shell began\n")
}

fn decimal(out: &mut [u8; 96], at: usize, v: u64) -> usize {
    let mut digits = [0u8; 20];
    let (mut i, mut v) = (digits.len(), v);
    loop {
        i -= 1;
        digits[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    put(out, at, &digits[i..])
}

fn put(out: &mut [u8; 96], at: usize, s: &[u8]) -> usize {
    let n = s.len().min(out.len() - at);
    out[at..at + n].copy_from_slice(&s[..n]);
    at + n
}
