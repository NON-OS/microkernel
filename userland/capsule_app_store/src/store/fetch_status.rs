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

//! Asking the model fetcher where the download it runs for a Qwen tier's
//! install stands (`status_wire`): the installer drops the fetcher's lines,
//! so the card asks the fetcher on its own endpoint, `tool.model-fetch`. The
//! answer is numbers only; the card is the tier whose files sum to its total.
//!
//! The fetcher answers between its own calls to the network services, and a
//! call to net.anon may take seconds while net.anon pumps a large stream. A
//! 150 ms wait on the window thread missed nearly every answer: the card
//! froze through the slowest part of an install and the serial filled with
//! unanswered calls. So the question is asked on a worker thread (libc's
//! `WorkerSeat`, as Settings asks the Wi-Fi driver), which waits longer than
//! the fetcher's longest call; the window keeps painting, and a tick picks
//! the answer up. Should no worker start, it is asked on the window thread
//! with the short wait, as before.

use nonos_libc::{Handoff, Look, WorkerSeat};

use crate::status_wire::Status;

#[path = "fetch_call.rs"]
mod fetch_call;

/* Past the fetcher's longest call to net.anon (CALL_MS, 20 s). */
const WORKER_WAIT_MS: u64 = 25_000;
const WINDOW_WAIT_MS: u64 = 150;
const EBUSY: i64 = -16;

static ANSWER: Handoff<Option<Status>> = Handoff::new();
static SEAT: WorkerSeat = WorkerSeat::new();

/// News from the fetcher, if any has come: `Some(answer)` when an ask has
/// finished (the answer itself `None` when the fetcher did not give one),
/// `None` while one is out. Starts the next ask when none is.
pub fn ask() -> Option<Option<Status>> {
    match ANSWER.poll() {
        Look::Ready(answer) => return Some(answer),
        Look::Waiting => return None,
        Look::Idle => {}
    }
    if !ANSWER.claim() {
        return None;
    }
    match SEAT.spawn(work, 0) {
        Ok(_) => None,
        Err(errno) => {
            ANSWER.release();
            // The last worker has answered and is on its way out.
            if errno == EBUSY {
                return None;
            }
            Some(fetch_call::call(WINDOW_WAIT_MS))
        }
    }
}

fn work(_: usize) {
    ANSWER.put(fetch_call::call(WORKER_WAIT_MS));
}
