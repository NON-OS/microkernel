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

//! OP_RX_BATCH off the real ring code: every completed frame lands in the
//! body net.core reads, in order, and every slot taken goes back to the part
//! once, errored ones included. Where a batch stops is in batch_limit_tests.

use crate::batch_ring::{body_frames, Ring};

#[test]
fn nothing_waiting_is_no_batch_and_no_slot_given_back() {
    let mut r = Ring::new();
    let mut gave = Vec::new();
    assert_eq!(r.take(24, &mut gave), None);
    assert!(gave.is_empty());
}

#[test]
fn every_waiting_frame_comes_in_order_with_its_slot_given_back() {
    let mut r = Ring::new();
    r.complete(0, b"first frame");
    r.complete(1, &[0xAB; 60]);
    r.complete(2, &[7; 1514]);
    let mut gave = Vec::new();
    let body = r.take(24, &mut gave).unwrap();
    let frames = body_frames(&body).unwrap();
    assert_eq!(frames, [b"first frame".to_vec(), vec![0xAB; 60], vec![7; 1514]]);
    assert_eq!(gave, [0, 1, 2]);
    assert_eq!(r.take(24, &mut gave), None);
}

#[test]
fn an_errored_frame_is_left_out_but_its_slot_still_goes_back() {
    let mut r = Ring::new();
    r.complete(0, b"good");
    r.fail(1);
    r.complete(2, b"also good");
    let mut gave = Vec::new();
    let frames = body_frames(&r.take(24, &mut gave).unwrap()).unwrap();
    assert_eq!(frames, [b"good".to_vec(), b"also good".to_vec()]);
    assert_eq!(gave, [0, 1, 2]);
}
