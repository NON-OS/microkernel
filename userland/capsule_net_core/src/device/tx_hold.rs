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

//! Frames the stack produced that the poll had no budget left to send.
//!
//! smoltcp counts a frame as sent once it has written it, so dropping one
//! here was invisible to TCP: a dropped ACK left the peer waiting on its
//! retransmit timer, a dropped data segment waited on ours. Frames are held
//! in order and go out first at the next poll.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use nonos_libc::mk_debug;
use spin::Mutex;

use super::budget;
use super::tx;

/// Frames held at most; past this the newest is dropped, and said so.
const HELD_MAX: usize = 64;

static HELD: Mutex<VecDeque<Vec<u8>>> = Mutex::new(VecDeque::new());

/// Send `frame` now if the poll allows and nothing older is waiting,
/// otherwise hold it behind what is.
pub fn send_or_hold(port: u32, frame: Vec<u8>) {
    let mut held = HELD.lock();
    if held.is_empty() && budget::may_send() {
        drop(held);
        tx::send_frame(port, &frame);
        return;
    }
    if held.len() >= HELD_MAX {
        let line = b"[NET-CORE] tx hold full, frame dropped\n";
        let _ = mk_debug(line.as_ptr(), line.len());
        return;
    }
    held.push_back(frame);
}

/// Send held frames, oldest first, while the poll allows.
pub fn flush(port: u32) {
    loop {
        let mut held = HELD.lock();
        if held.is_empty() || !budget::may_send() {
            return;
        }
        let frame = held.pop_front();
        drop(held);
        if let Some(frame) = frame {
            tx::send_frame(port, &frame);
        }
    }
}
