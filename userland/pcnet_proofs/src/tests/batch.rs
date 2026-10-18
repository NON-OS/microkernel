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

//! `OP_RX_BATCH` bodies: a count, then each frame behind its length, at most
//! BATCH_MAX_FRAMES at once, with dropped frames freeing their slots.

use super::util::up;
use crate::bench::Bench;
use crate::model::with_part;
use crate::protocol::BATCH_MAX_FRAMES;
use crate::rx_fill::fill;

fn frames(body: &[u8]) -> Vec<Vec<u8>> {
    let word = |at: usize| u32::from_le_bytes(body[at..at + 4].try_into().unwrap()) as usize;
    let (mut out, mut at) = (Vec::new(), 4);
    for _ in 0..word(0) {
        let len = word(at);
        out.push(body[at + 4..at + 4 + len].to_vec());
        at += 4 + len;
    }
    assert_eq!(at, body.len(), "nothing after the last frame");
    out
}

#[test]
fn a_batch_carries_the_good_frames_and_frees_every_slot() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    assert!(fill(&mut d.rx).is_none());
    with_part(|p| {
        p.deliver(&[1; 60], false);
        p.deliver(&[2; 70], true);
        p.deliver(&[3; 80], false);
    });
    let body = fill(&mut d.rx).expect("a batch");
    assert_eq!(frames(&body), vec![vec![1; 60], vec![3; 80]]);
    let rdra = with_part(|p| p.rings.rdra);
    for i in 0..3 {
        assert_eq!(with_part(|p| p.mem.u16(rdra + i * 16 + 6)), 0x8000, "slot {i}");
    }
}

#[test]
fn a_batch_stops_at_its_frame_limit_and_the_rest_wait() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    for n in 0..30u8 {
        with_part(|p| p.deliver(&[n; 100], false));
    }
    let first = frames(&fill(&mut d.rx).expect("first"));
    let second = frames(&fill(&mut d.rx).expect("second"));
    assert_eq!((first.len(), second.len()), (BATCH_MAX_FRAMES, 30 - BATCH_MAX_FRAMES));
    assert_eq!(second[0], vec![BATCH_MAX_FRAMES as u8; 100]);
    assert!(fill(&mut d.rx).is_none());
}
