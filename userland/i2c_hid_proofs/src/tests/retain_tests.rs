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

//! No input is kept once it is delivered (plan/50 C8): the buffer a frame
//! is read into is cleared after its report is posted, the reply buffer of
//! the transfer it came through too, and the repeat check keeps the decoded
//! pointer sample, not the report's bytes.

use nonos_i2cmodel::touchpad::Touch;
use nonos_libc::take_events;

use super::driven::{bound, frame};
use crate::i2c_client::transfer::receive;
use crate::state::{Seen, FRAME_MAX};

#[test]
fn a_delivered_frame_leaves_its_read_buffer_cleared() {
    let (r, mut s) = bound();
    frame(&r, &mut s, Touch::finger(0x0123, 0x0156));
    frame(&r, &mut s, Touch::finger(0x0133, 0x0156));
    assert!(!take_events().is_empty(), "the frames were not delivered");
    assert_eq!(s.frame.len(), FRAME_MAX, "the buffer went with the frame");
    assert!(s.frame.iter().all(|&b| b == 0), "the frame is still in the buffer");
    let Some(Seen::Touch(t)) = s.repeat.last else { panic!("the repeat check saw no touch") };
    assert_eq!((t.x, t.y), (0x0133, 0x0156));
}

#[test]
fn a_report_seen_again_is_still_taken_for_a_repeat() {
    let (r, mut s) = bound();
    let rest = Touch::finger(600, 400);
    frame(&r, &mut s, rest);
    let first = s.repeat.last;
    frame(&r, &mut s, rest);
    assert!(s.repeat.last == first && s.repeat.frame_repeats == 1);
    frame(&r, &mut s, Touch::finger(601, 400));
    assert!(s.repeat.last != first && s.repeat.frame_repeats == 0, "a change was a repeat");
}

#[test]
fn the_reply_a_report_came_back_in_is_cleared_once_it_is_copied_out() {
    let seq = 0x5EED_u64;
    let mut rx = vec![0u8; 32 + 4];
    rx[0..4].copy_from_slice(&0x4E49_3243u32.to_le_bytes());
    rx[8..16].copy_from_slice(&seq.to_le_bytes());
    rx[24..26].copy_from_slice(&4u16.to_le_bytes());
    rx[32..36].copy_from_slice(&[6, 0, 3, 0x2A]);
    let mut read = [0u8; 4];
    assert_eq!(receive(&mut rx, 36, seq, &mut read), Some(4));
    assert_eq!(read, [6, 0, 3, 0x2A], "the report was not copied out");
    assert!(rx.iter().all(|&b| b == 0), "the reply still holds the report");
}
