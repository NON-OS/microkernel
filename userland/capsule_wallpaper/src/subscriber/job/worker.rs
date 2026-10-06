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

//! The job, on a worker thread.
//!
//! The wallpaper service has one thread, and while it sat in a catalog call
//! (a wallpaper is 25 to 120 of them, the first allowed 15 s) or in the
//! decode, nobody answered the service: the desktop shell's calls timed out
//! and its setup stuck on "wallpaper call failed". The fetch and the decode
//! now run here (libc's `WorkerSeat`), and the service thread picks up the
//! picture on a tick (`Handoff`) and paints it itself, so every call to the
//! compositor stays on the thread that registered the surface (the
//! compositor knows a layer by the pid that submitted it).
//!
//! The kernel admits a thread under its process (4e4c172a): the worker
//! reaches wallpaper_catalog where the service does. Neither the catalog nor
//! the wallpaper is held to a peer list (src/services/registry/peers.rs,
//! held.rs), and the catalog answers whoever calls through the kernel's
//! reply path, which delivers to the worker's own reply inbox.
//!
//! The answer lands in a static, and the bytes a stopped job kept travel
//! back in it, so nothing is lost when a try fails.

use alloc::boxed::Box;

use nonos_libc::{Handoff, Look, WorkerSeat};

use super::fetch::fetch;
use super::plan::Outcome;
use crate::catalog_client::Download;
use crate::paint::DecodedImage;

/// What a job brought back, for wallpaper `index`.
pub struct Answer {
    pub index: u8,
    pub outcome: Outcome<Download, DecodedImage>,
}

/// What the worker is asked.
struct Ask {
    catalog_port: u32,
    index: u8,
    kept: Option<Download>,
}

/// What became of a job handed to `send`.
pub enum Sent {
    /// A worker has it; a tick will find the answer.
    Out,
    /// The last worker is still on its way out: try again next tick. The
    /// bytes kept come back.
    Busy(Option<Download>),
    /// No worker could be started (the kernel refused the thread, or its
    /// stack could not be mapped). The bytes kept come back.
    Refused(Option<Download>),
}

static ANSWER: Handoff<Answer> = Handoff::new();
static SEAT: WorkerSeat = WorkerSeat::new();

/// The seat's refusal while its last worker is still on it.
const EBUSY: i64 = -16;

/// Start a job for wallpaper `index`.
pub fn send(catalog_port: u32, index: u8, kept: Option<Download>) -> Sent {
    if !ANSWER.claim() {
        return Sent::Busy(kept);
    }
    let arg = Box::into_raw(Box::new(Ask { catalog_port, index, kept }));
    match SEAT.spawn(work, arg as usize) {
        Ok(_) => Sent::Out,
        Err(errno) => {
            // SAFETY: the spawn failed, so no worker took the box; it is
            // still this thread's.
            let ask = unsafe { Box::from_raw(arg) };
            ANSWER.release();
            if errno == EBUSY {
                return Sent::Busy(ask.kept);
            }
            Sent::Refused(ask.kept)
        }
    }
}

/// The answer, once a worker has put it in.
pub fn poll() -> Look<Answer> {
    ANSWER.poll()
}

fn work(arg: usize) {
    // SAFETY: `arg` is the box `send` leaked for this worker, and the spawn
    // succeeded, so the worker owns it from here.
    let ask = unsafe { Box::from_raw(arg as *mut Ask) };
    let Ask { catalog_port, index, kept } = *ask;
    ANSWER.put(Answer { index, outcome: fetch(catalog_port, index, kept) });
}
