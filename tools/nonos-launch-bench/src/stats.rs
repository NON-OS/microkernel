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

//! Repeats a closure and reports the median and p90 wall time.

use std::time::Instant;

pub const REPEATS: usize = 31;

pub struct Spread {
    pub median_ns: u128,
    pub p90_ns: u128,
}

pub fn time<F: FnMut()>(mut f: F) -> Spread {
    let mut t: Vec<u128> = (0..REPEATS)
        .map(|_| {
            let s = Instant::now();
            f();
            s.elapsed().as_nanos()
        })
        .collect();
    t.sort_unstable();
    Spread { median_ns: t[REPEATS / 2], p90_ns: t[REPEATS * 9 / 10] }
}

pub fn us(ns: u128) -> String {
    format!("{}.{:01}", ns / 1000, (ns % 1000) / 100)
}

pub fn row(what: &str, n: usize, s: &Spread) {
    println!("{what:<44} n={n:<4} median={:>10} us  p90={:>10} us  host", us(s.median_ns), us(s.p90_ns));
}
