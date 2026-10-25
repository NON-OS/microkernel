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

//! Random log lines with values planted in them, and the list of what was planted.

use crate::rng::Rng;

const WORDS: [&str; 12] = [
    "[NET]",
    "driver.e1000e0",
    "up",
    "8086:15b8",
    "00:1f.3",
    "status",
    "=",
    "ip:",
    "v",
    ",",
    "0x",
    "[USB]",
];

fn secret(rng: &mut Rng) -> String {
    let d = |rng: &mut Rng| rng.below(256);
    match rng.below(7) {
        0 => (0..6).map(|_| rng.hex(2)).collect::<Vec<_>>().join(":"),
        1 => (0..6).map(|_| rng.hex(2)).collect::<Vec<_>>().join("-"),
        2 => format!("{}.{}.{}.{}", d(rng), d(rng), d(rng), d(rng)),
        3 => format!("fe80::{}:{}:{}", rng.hex(4), rng.hex(4), rng.hex(4)),
        4 => [8, 4, 4, 4, 12].iter().map(|n| rng.hex(*n)).collect::<Vec<_>>().join("-"),
        5 => {
            let n = 12 + rng.below(40);
            rng.hex(n)
        }
        _ => {
            let n = 12 + rng.below(8);
            (0..n).map(|_| char::from(b'0' + rng.below(10) as u8)).collect()
        }
    }
}

pub fn line(rng: &mut Rng, planted: &mut Vec<String>) -> String {
    let mut out = String::new();
    for _ in 0..1 + rng.below(8) {
        if rng.below(3) == 0 {
            let s = secret(rng);
            out.push_str(&s);
            planted.push(s);
        } else {
            out.push_str(WORDS[rng.below(WORDS.len())]);
        }
        if rng.below(2) == 0 {
            out.push(' ');
        }
    }
    out
}
