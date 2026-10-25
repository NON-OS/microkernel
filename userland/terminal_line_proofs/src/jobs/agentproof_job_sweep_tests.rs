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

//! Any bytes as the headless run's job: refused, or one line of at most
//! eight words with no quote, redirect, pipe or separator in it.

use super::agentproof_job::line_of;

#[test]
fn any_bytes_are_refused_or_one_safe_line() {
    let pieces: [&[u8]; 14] = [
        b"nonos-agentproof-job 1\n",
        b"box strict\n",
        b"run sh /a\n",
        b"box ",
        b"run ",
        b"strict",
        b"\n",
        b" ",
        b"sh",
        b"'",
        b">",
        b"|",
        b"\0",
        b"\xff",
    ];
    let mut x: u64 = 0x5eed_0b0c;
    let mut taken = 0;
    for _ in 0..200_000 {
        let mut t = Vec::new();
        for _ in 0..(x % 6) + 2 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            t.extend_from_slice(pieces[(x % 14) as usize]);
        }
        x ^= x << 13;
        if let Ok(line) = line_of(&t) {
            taken += 1;
            let words: Vec<&[u8]> = line.split(|b| *b == b' ').collect();
            assert!(line.starts_with(b"linux --box ") && words.len() <= 8);
            assert!(line.iter().all(|b| b.is_ascii_graphic() || *b == b' '));
            assert!(!line.iter().any(|b| b"'\"<>|;&`$\\".contains(b)));
        }
    }
    assert!(taken > 0);
}
