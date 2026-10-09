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

//! Taking received frames from the driver a batch at a time.
//!
//! One frame per round trip, in an 8 ms window one round trip on an emulated
//! CPU nearly fills, took in two frames per poll: 9 KB/s whatever the window.
//! A driver serving OP_RX_BATCH hands over every frame it holds in one
//! numbered reply, and the same batch for the same number, so a reply that
//! comes after the call gave up is not lost.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use nonos_libc::mk_debug;
use spin::Mutex;

use super::batch_call::call;
use super::batch_frames::batch_frames;

static SEQ: AtomicU32 = AtomicU32::new(1);
static ASKED_THIS_POLL: AtomicBool = AtomicBool::new(false);
static QUEUE: Mutex<VecDeque<Vec<u8>>> = Mutex::new(VecDeque::new());

/// Drop whatever frames are queued; the NIC they came from is gone.
pub(super) fn forget_queue() {
    QUEUE.lock().clear();
}

/// A poll is starting; it may ask the driver once more.
pub fn begin_poll() {
    ASKED_THIS_POLL.store(false, Ordering::Relaxed);
}

/// The next received frame: from the batch in hand, or from one new batch
/// asked for once per poll.
pub fn next(port: u32) -> Option<Vec<u8>> {
    if let Some(frame) = QUEUE.lock().pop_front() {
        return Some(frame);
    }
    if ASKED_THIS_POLL.swap(true, Ordering::Relaxed) {
        return None;
    }
    let seq = SEQ.load(Ordering::Relaxed);
    if let Some(body) = call(port, Some(seq)) {
        /* Moved past even when unreadable, or the driver would give the same
         * body again for ever; the loss is named, not silent. */
        SEQ.store(seq.wrapping_add(1).max(1), Ordering::Relaxed);
        match batch_frames(&body) {
            Some(frames) => QUEUE.lock().extend(frames),
            None => say(b"[NET-CORE] rx batch unreadable, dropped\n"),
        }
    }
    QUEUE.lock().pop_front()
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
