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

//! Frame assembly over seeded payload streams, built sound and then told
//! one lie: the assembler hands up exactly the frames the rule in
//! payload_sweep/model.rs does, at the payload it does, and exactly the
//! honest frames the lie leaves, each whole and at most `max` bytes.

use nonos_uvc::{Assembler, Payload};

use crate::payload_sweep::build::stream;
use crate::payload_sweep::lie::{tell, LIES};
use crate::payload_sweep::model;
use crate::payload_sweep::rng::{rounds, Rng};

fn driver(payloads: &[Vec<u8>], max: usize) -> Vec<(usize, Vec<u8>)> {
    let mut a = Assembler::new(max);
    let mut out = Vec::new();
    for (i, p) in payloads.iter().enumerate() {
        if let Payload::Frame = a.push(p) {
            out.push((i, a.frame().to_vec()));
        }
    }
    out
}

#[test]
fn every_sound_stream_hands_up_every_frame_whole() {
    for seed in 1..rounds(20_000) {
        let s = stream(&mut Rng::new(seed));
        let p: Vec<Vec<u8>> = s.payloads.iter().map(|p| p.bytes.clone()).collect();
        let got: Vec<Vec<u8>> = driver(&p, s.max).into_iter().map(|f| f.1).collect();
        assert_eq!(got, s.frames, "seed {seed}");
    }
}

#[test]
fn after_any_lie_exactly_the_honest_frames_are_handed_up() {
    let mut told = [0u32; LIES.len()];
    for seed in 1..rounds(60_000) {
        let t = tell(&mut Rng::new(seed));
        let p: Vec<Vec<u8>> = t.s.payloads.iter().map(|p| p.bytes.clone()).collect();
        let got = driver(&p, t.s.max);
        let why = format!("seed {seed}: {:?}", t.lie);
        assert_eq!(got, model::frames(&p, t.s.max), "{why}");
        let frames: Vec<Vec<u8>> = got.into_iter().map(|f| f.1).collect();
        assert_eq!(frames, t.expect, "{why}: the honest frames left");
        told[LIES.iter().position(|l| *l == t.lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 3_000, "{lie:?} was told only {n} times");
    }
}
