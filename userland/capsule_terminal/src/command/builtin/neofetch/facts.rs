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

//! The figures `neofetch` prints, from what the kernel reports and nothing
//! else: a figure it could not read says `unknown`, never a guess.

use alloc::vec::Vec;

use crate::term::util::format_u64;

/// `n` tenths as `12.3`.
fn tenths(n: u64, out: &mut Vec<u8>) {
    let mut d = [0u8; 20];
    let k = format_u64(n / 10, &mut d);
    out.extend_from_slice(&d[..k]);
    out.push(b'.');
    out.push(b'0' + (n % 10) as u8);
}

/// Resident memory over the machine's: `1.9 GiB / 7.0 GiB (27%)`.
pub fn memory(used_kb: Option<u64>, total_kb: Option<u64>) -> Vec<u8> {
    let mut out = Vec::new();
    let gib = |kb: u64, out: &mut Vec<u8>| {
        tenths((kb * 10 + 524_288) / 1_048_576, out);
        out.extend_from_slice(b" GiB");
    };
    match (used_kb, total_kb) {
        (Some(u), Some(t)) if t > 0 => {
            gib(u, &mut out);
            out.extend_from_slice(b" / ");
            gib(t, &mut out);
            let mut d = [0u8; 20];
            let k = format_u64((u * 100 + t / 2) / t, &mut d);
            out.extend_from_slice(b" (");
            out.extend_from_slice(&d[..k]);
            out.extend_from_slice(b"%)");
        }
        _ => out.extend_from_slice(b"unknown"),
    }
    out
}

/// The scheduler's 1, 5 and 15 minute averages, Q11 fixed point: `0.42 0.31 0.20`.
pub fn load(q11: Option<[u64; 3]>) -> Vec<u8> {
    let Some(l) = q11 else { return Vec::from(&b"unknown"[..]) };
    let mut out = Vec::new();
    for (i, v) in l.iter().enumerate() {
        if i > 0 {
            out.push(b' ');
        }
        let hundredths = (v * 100 + 1024) / 2048;
        let mut d = [0u8; 20];
        let k = format_u64(hundredths / 100, &mut d);
        out.extend_from_slice(&d[..k]);
        out.push(b'.');
        out.push(b'0' + (hundredths / 10 % 10) as u8);
        out.push(b'0' + (hundredths % 10) as u8);
    }
    out
}
