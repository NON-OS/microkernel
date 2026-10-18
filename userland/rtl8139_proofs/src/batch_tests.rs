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

//! OP_RX_BATCH's gather, over a scripted `recv_one`: the frames come in the
//! body net.core reads, in order; it stops at the frame limit, at an empty
//! part and at the first error; and an error after frames keeps both the
//! frames and the error, never one at the cost of the other.

use crate::batch_script::run;

#[test]
fn nothing_waiting_is_no_batch_and_no_error() {
    assert_eq!(run(16, vec![]), (None, None, 1));
}

#[test]
fn every_waiting_frame_comes_in_order() {
    let steps = vec![Ok(Some(b"one".to_vec())), Ok(Some(vec![9; 1514])), Ok(None)];
    assert_eq!(run(16, steps), (Some(vec![b"one".to_vec(), vec![9; 1514]]), None, 3));
}

#[test]
fn the_frame_limit_ends_the_batch_and_the_rest_wait() {
    let steps = (0..5).map(|i| Ok(Some(vec![i; 60]))).collect();
    let (body, err, calls) = run(3, steps);
    assert_eq!((body.unwrap().len(), err, calls), (3, None, 3));
}

#[test]
fn an_error_before_any_frame_is_reported_alone() {
    assert_eq!(run(16, vec![Err("rx ring restarted")]), (None, Some("rx ring restarted"), 1));
}

#[test]
fn an_error_after_frames_keeps_the_frames_and_the_error() {
    let steps =
        vec![Ok(Some(b"kept".to_vec())), Err("frame too large"), Ok(Some(b"later".to_vec()))];
    assert_eq!(run(16, steps), (Some(vec![b"kept".to_vec()]), Some("frame too large"), 2));
}
