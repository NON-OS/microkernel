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

//! The firmware's event log as About's Verify screen names it: the evidence
//! on "machine recorded everything it runs", since the log is where the
//! firmware measured what it booted (attest document, `Doc::log_len`).

/// "event log 4096 bytes", or "no event log" when the loader carried none,
/// written into `out`; its length. The longest is 26 bytes.
pub fn log_words(len: u32, out: &mut [u8; 40]) -> usize {
    if len == 0 {
        return put(out, 0, b"no event log");
    }
    let mut digits = [0u8; 10];
    let (mut at, mut v) = (digits.len(), len);
    while v > 0 {
        at -= 1;
        digits[at] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    let n = put(out, 0, b"event log ");
    let n = put(out, n, &digits[at..]);
    put(out, n, if len == 1 { b" byte" } else { b" bytes" })
}

fn put(out: &mut [u8; 40], at: usize, s: &[u8]) -> usize {
    out[at..at + s.len()].copy_from_slice(s);
    at + s.len()
}
