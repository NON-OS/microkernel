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
//! Frames from payloads: handed up whole once their end is seen, and
//! dropped when marked bad, cut short by a new frame, or past their bound.

use nonos_uvc::{Assembler, Payload};

/// A payload: a 2-byte header with `info`, then `data`.
fn p(info: u8, data: &[u8]) -> Vec<u8> {
    let mut v = vec![2, info | 0x80];
    v.extend_from_slice(data);
    v
}

fn done(a: &mut Assembler, pay: &[u8]) -> bool {
    matches!(a.push(pay), Payload::Frame)
}

#[test]
fn frames_come_whole_and_alternate_their_fid() {
    let mut a = Assembler::new(16);
    assert!(!done(&mut a, &p(0, &[1, 2, 3])));
    assert!(!done(&mut a, &p(0, &[])), "an empty payload adds nothing");
    assert!(done(&mut a, &p(0x2, &[4, 5])));
    assert_eq!(a.frame(), &[1, 2, 3, 4, 5]);
    assert!(done(&mut a, &p(0x1 | 0x2, &[9])));
    assert_eq!(a.frame(), &[9]);
}

#[test]
fn a_bad_a_cut_or_an_oversized_frame_is_dropped() {
    let mut a = Assembler::new(4);
    assert!(done(&mut a, &p(0x2, &[7])));
    assert!(!done(&mut a, &p(0x1 | 0x40, &[1])) && !done(&mut a, &p(0x1 | 0x2, &[2])), "ERR");
    assert!(!done(&mut a, &p(0x0, &[3, 3])), "a frame begun");
    assert!(done(&mut a, &p(0x1 | 0x2, &[4])), "a new FID drops the cut one");
    assert_eq!(a.frame(), &[4]);
    assert!(!done(&mut a, &p(0x2, &[1, 2, 3, 4, 5])), "past 4 bytes");
    assert!(!done(&mut a, &[9, 0x82, 1]), "a header longer than its payload");
    // That payload's FID could not be read: it may have begun the next frame.
    assert!(!done(&mut a, &p(0x1 | 0x2, &[6])), "the frame after it begins spoiled");
    assert!(done(&mut a, &p(0x2, &[8])));
    assert_eq!(a.frame(), &[8], "the last whole frame only");
}
