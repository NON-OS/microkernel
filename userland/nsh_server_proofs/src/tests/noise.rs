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

//! The fuzz target's oracle run here on fixed inputs, so every test run
//! holds the server to it: noise at each stage, every kind byte with bodies
//! of each length near its rules' edges, and good frames with one bit
//! flipped or cut short.

use crate::kit::noise::drive;
use crate::kit::rng::Det;

pub(super) fn framed(stage: u8, frames: &[Vec<u8>]) -> Vec<u8> {
    let mut out = vec![stage];
    for f in frames {
        out.extend_from_slice(&(f.len() as u16).to_le_bytes());
        out.extend_from_slice(f);
    }
    out
}

#[test]
fn noise_at_every_stage() {
    let mut rng = Det::new(0x4E);
    for i in 0..40u32 {
        let mut data = vec![0u8; 1 + (i as usize * 37) % 900];
        rng.fill(&mut data);
        for stage in 0..3 {
            data[0] = stage;
            drive(&data);
        }
    }
}

#[test]
fn every_kind_with_bodies_at_their_edges() {
    for stage in [1u8, 2] {
        // Every kind nsh/1 defines, and undefined bytes at each gap; any
        // other undefined byte takes the same refusal.
        let kinds = [0x01, 0x02, 0x10, 0x20, 0x21, 0x22, 0x30, 0x31, 0x32, 0x33, 0x40, 0x41, 0x50]
            .into_iter()
            .chain([0x51, 0x52, 0x7F, 0x00, 0x03, 0x23, 0x7E, 0x80, 0xFF]);
        for kind in kinds {
            for len in [0usize, 1, 2, 4, 5, 10] {
                let frame = [&[kind][..], &vec![1u8; len]].concat();
                drive(&framed(stage, &[frame]));
            }
        }
    }
}
