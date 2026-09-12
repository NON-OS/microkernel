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

//! One summary as one line.

use nonos_bench_core::Summary;

use crate::command::output::Output;

/// Column starts, so the four figures line up under their heading without a
/// table engine.
const COLS: [usize; 5] = [10, 20, 30, 40, 50];

pub fn heading(out: &mut Output<'_>) {
    let mut line = [b' '; 62];
    put(&mut line, 0, b"PROBE");
    put(&mut line, COLS[0], b"p50");
    put(&mut line, COLS[1], b"p95");
    put(&mut line, COLS[2], b"p99");
    put(&mut line, COLS[3], b"max");
    put(&mut line, COLS[4], b"cycles");
    out.writeln(&line);
}

pub fn row(out: &mut Output<'_>, name: &[u8], s: &Summary) {
    let mut line = [b' '; 62];
    put(&mut line, 0, name);
    for (i, v) in [s.p50, s.p95, s.p99, s.max].into_iter().enumerate() {
        let mut b = [0u8; 20];
        let n = decimal(v, &mut b);
        put(&mut line, COLS[i], &b[..n]);
    }
    out.writeln(&line);
}

fn put(dst: &mut [u8], at: usize, src: &[u8]) {
    let end = (at + src.len()).min(dst.len());
    if at < end {
        dst[at..end].copy_from_slice(&src[..end - at]);
    }
}

pub fn decimal(mut v: u64, out: &mut [u8; 20]) -> usize {
    if v == 0 {
        out[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 20];
    let mut n = 0;
    while v > 0 {
        tmp[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    for i in 0..n {
        out[i] = tmp[n - 1 - i];
    }
    n
}
