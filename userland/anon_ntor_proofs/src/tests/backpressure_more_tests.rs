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
//! Backpressure, continued.

use crate::circuit::window::{STREAM_INCREMENT, STREAM_START};
use crate::stream::{Stream, StreamStage};

const HIGH_WATER: usize = STREAM_INCREMENT as usize * 498;

fn arrived(stream: &mut Stream, cells: i32, bytes_each: usize) {
    for _ in 0..cells {
        stream.deliver_window -= 1;
        stream.delivered_since += 1;
        stream.inbound.extend(core::iter::repeat_n(0u8, bytes_each));
    }
}

#[test]
fn draining_releases_a_withheld_grant() {
    let mut stream = Stream::new(1, 7);
    stream.stage = StreamStage::Open;
    arrived(&mut stream, STREAM_INCREMENT + 1, 498);
    assert!(!stream.take_sendme_due(HIGH_WATER));
    let taken = stream.inbound.len();
    stream.inbound.drain(..taken);
    assert!(stream.take_sendme_due(HIGH_WATER), "the grant it still owed goes out now");
}
#[test]
fn a_partial_window_owes_nothing_either_way() {
    let mut stream = Stream::new(1, 7);
    stream.stage = StreamStage::Open;
    arrived(&mut stream, STREAM_INCREMENT - 1, 498);
    assert!(!stream.take_sendme_due(HIGH_WATER), "not due yet");
    assert!(!stream.take_sendme_due(usize::MAX), "still not due with no mark at all");
}
