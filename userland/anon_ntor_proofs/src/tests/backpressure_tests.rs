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
//! Backpressure.

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
fn a_drained_stream_is_granted_room() {
    let mut stream = Stream::new(1, 7);
    stream.stage = StreamStage::Open;
    arrived(&mut stream, STREAM_INCREMENT, 498);
    stream.inbound.clear();
    assert!(stream.take_sendme_due(HIGH_WATER), "the caller kept up, so the window reopens");
    assert_eq!(stream.deliver_window, STREAM_START, "back to a full window");
    assert_eq!(stream.delivered_since, 0, "and the count is settled");
}
#[test]
fn a_stream_nobody_reads_is_not_granted_room() {
    let mut stream = Stream::new(1, 7);
    stream.stage = StreamStage::Open;
    // One increment lands exactly on the mark, which is allowed: it is what a
    // single grant brings in, and refusing there would stall a reader keeping
    // pace. One cell past it is the caller genuinely falling behind.
    arrived(&mut stream, STREAM_INCREMENT + 1, 498);
    assert!(stream.inbound.len() > HIGH_WATER, "past the mark, not on it");
    assert!(!stream.take_sendme_due(HIGH_WATER), "no grant while the caller is behind");
    assert_eq!(
        stream.delivered_since,
        STREAM_INCREMENT + 1,
        "and the debt is kept, not forgiven, so the grant still goes out once drained"
    );
}
