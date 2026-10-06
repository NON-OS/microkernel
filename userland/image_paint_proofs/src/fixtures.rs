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

//! Fixture files and their reference decodes (fixtures/expect.txt: status,
//! width, height and an FNV-1a 64 of the ARGB words, fully transparent
//! pixels counted as 0, from an independent decoder).
use std::string::String;
use std::vec::Vec;

pub fn read(rel: &str) -> Vec<u8> {
    let p = std::format!("{}/fixtures/{}", env!("CARGO_MANIFEST_DIR"), rel);
    std::fs::read(&p).unwrap_or_else(|e| panic!("fixture {p}: {e}"))
}

/* Width, height and pixel hash, or None where the reference refuses the file. */
pub type Expect = Option<(u32, u32, u64)>;

pub fn expectations() -> Vec<(String, Expect)> {
    let text = String::from_utf8(read("expect.txt")).expect("utf-8 manifest");
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
        let f: Vec<&str> = line.split_whitespace().collect();
        let dims = (f[1] == "ok").then(|| {
            (f[2].parse().unwrap(), f[3].parse().unwrap(), u64::from_str_radix(f[4], 16).unwrap())
        });
        out.push((String::from(f[0]), dims));
    }
    out
}

pub fn fnv(px: &[u32]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &p in px {
        let p = if p >> 24 == 0 { 0 } else { p };
        for b in p.to_le_bytes() {
            h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}
