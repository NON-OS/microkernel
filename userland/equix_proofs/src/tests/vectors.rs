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


//! equix.expect, read into typed records. The file is the fork's own output;
//! see vectors/equix_gen.c for how it was made.

use crate::hex::hex;
use nonos_equix::Solution;

const EXPECT: &str = include_str!("../../vectors/equix.expect");

pub struct Blake {
    pub out_len: usize,
    pub salted: bool,
    pub input: Vec<u8>,
    pub digest: Vec<u8>,
}

pub struct HashLine {
    pub seed: Vec<u8>,
    pub input: u64,
    pub output: [u8; 8],
}

pub struct SolveLine {
    pub challenge: Vec<u8>,
    pub solutions: Vec<Solution>,
}

pub struct PowLine {
    pub seed: [u8; 32],
    pub blinded: [u8; 32],
    pub first_nonce: [u8; 16],
    pub effort: u32,
    pub nonce: [u8; 16],
    pub solution: [u8; 16],
    pub solves: u32,
}

fn lines(tag: &str) -> impl Iterator<Item = Vec<&'static str>> + '_ {
    EXPECT
        .lines()
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .filter(move |f| f[0] == tag)
}

fn named_input(name: &str) -> Vec<u8> {
    let ramp: Vec<u8> = (0..300u32).map(|i| i as u8).collect();
    match name {
        "abc" => b"abc".to_vec(),
        "empty" => Vec::new(),
        "ramp128" => ramp[..128].to_vec(),
        "ramp129" => ramp[..129].to_vec(),
        "ramp300" => ramp,
        other => panic!("unknown input {other}"),
    }
}

pub fn blake() -> Vec<Blake> {
    // "b2 <input> <len> <salt> <digest>", where the salt may contain a space.
    lines("b2")
        .map(|f| Blake {
            input: named_input(f[1]),
            out_len: f[2].parse().unwrap(),
            salted: f[3] != "-",
            digest: hex(f[f.len() - 1]),
        })
        .collect()
}

pub fn hashes() -> Vec<HashLine> {
    lines("hashx")
        .map(|f| HashLine {
            seed: if f[1] == "-" { Vec::new() } else { hex(f[1]) },
            input: f[2].parse().unwrap(),
            output: hex(f[3]).try_into().unwrap(),
        })
        .collect()
}

pub fn seeds() -> Vec<(Vec<u8>, bool)> {
    lines("seed").map(|f| (hex(f[1]), f[2] == "ok")).collect()
}

pub fn solves() -> Vec<SolveLine> {
    lines("solve")
        .map(|f| {
            let count: usize = f[2].parse().unwrap();
            let solutions: Vec<Solution> = f[3..]
                .iter()
                .map(|s| Solution::from_bytes(&hex(s).try_into().unwrap()))
                .collect();
            assert_eq!(solutions.len(), count);
            SolveLine { challenge: hex(f[1]), solutions }
        })
        .collect()
}

pub fn pows() -> Vec<PowLine> {
    lines("pow")
        .map(|f| PowLine {
            seed: hex(f[1]).try_into().unwrap(),
            blinded: hex(f[2]).try_into().unwrap(),
            first_nonce: hex(f[3]).try_into().unwrap(),
            effort: f[4].parse().unwrap(),
            nonce: hex(f[5]).try_into().unwrap(),
            solution: hex(f[6]).try_into().unwrap(),
            solves: f[7].parse().unwrap(),
        })
        .collect()
}
