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

//! `pkg install` and `pkg remove` as a terminal job whose installer call
//! waits on a worker thread, so the window keeps painting and reading keys
//! while a multi-megabyte package is verified.
//!
//! The installer's answer is the reply to one call (up to 30 s), and a reply
//! that comes after its call gave up is dropped, so the call cannot be cut
//! into ticks. The worker makes it and hands the answer back (libc's `Handoff`);
//! the job looks for it each tick. Ctrl+C ends the job at once; the worker
//! still finishes its call and its answer is discarded. One worker at a time:
//! a request while the last one is still out is refused and said so.
//!
//! The installer answers whoever calls through the kernel's reply path and
//! does not check which process asks (`capsule_installer/src/server`), so the
//! worker's own thread id is no obstacle.

use nonos_libc::{Handoff, WorkerSeat};

use super::work::PkgDone;

#[path = "job_prepare.rs"]
mod job_prepare;
#[path = "job_say.rs"]
mod job_say;
#[path = "job_step.rs"]
mod job_step;

pub use job_prepare::prepare;

/// Where the worker's answer lands.
static ANSWER: Handoff<PkgDone> = Handoff::new();
/// The worker's thread and its stack.
static SEAT: WorkerSeat = WorkerSeat::new();

pub struct PkgJob {
    /// The answer was taken; nothing is left to stop waiting for.
    answered: bool,
}

/// What `prepare` made of the request.
pub enum Prepared {
    /// A worker is on it.
    Job(PkgJob),
    /// Refused and said so.
    Refused,
    /// Not a request for a worker (`status`, or a usage error), or no worker
    /// could be started: the ordinary dispatch runs it, on this thread.
    Inline,
}
