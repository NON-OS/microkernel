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

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_libc::{Handoff, Look, WorkerSeat};

use super::work::{parse, perform, report, PkgDone, PkgOp};
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::term::state::State;
use crate::term::util::format_u64;

/// Where the worker's answer lands.
static ANSWER: Handoff<PkgDone> = Handoff::new();
/// The worker's thread and its stack.
static SEAT: WorkerSeat = WorkerSeat::new();

/// The seat's refusal while its last worker is still on it.
const EBUSY: i64 = -16;

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

pub fn prepare(state: &mut State, args: &[&[u8]]) -> Prepared {
    let Some(op) = parse(state.cwd.as_bytes(), args) else {
        return Prepared::Inline;
    };
    if !ANSWER.claim() {
        return refuse(state);
    }
    let verifying = match &op {
        PkgOp::Install { path, .. } => Some(said(b"pkg: verifying ", path)),
        PkgOp::Remove { .. } => None,
    };
    let arg = Box::into_raw(Box::new(op));
    match SEAT.spawn(work, arg as usize) {
        Ok(_) => {
            if let Some(line) = verifying {
                state.scrollback.push_line(&line);
            }
            Prepared::Job(PkgJob { answered: false })
        }
        Err(errno) => {
            // SAFETY: the spawn failed, so no worker took the box; it is
            // still this thread's, and only this thread frees it.
            drop(unsafe { Box::from_raw(arg) });
            ANSWER.release();
            if errno == EBUSY {
                // The last worker has handed its answer back but is not gone
                // yet, so its stack is still in use.
                return refuse(state);
            }
            say_inline(state, errno);
            Prepared::Inline
        }
    }
}

/// The worker: ask the installer, hand the answer back, end.
fn work(arg: usize) {
    // SAFETY: `arg` is the box `prepare` leaked for this worker, and the
    // spawn succeeded, so the worker owns it from here.
    let op = unsafe { Box::from_raw(arg as *mut PkgOp) };
    ANSWER.put(perform(&op));
}

impl PkgJob {
    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        match ANSWER.poll() {
            Look::Waiting => JobProgress::Running,
            Look::Ready(done) => {
                self.answered = true;
                JobProgress::Done(if report(out, &done) { 0 } else { 1 })
            }
            Look::Idle => {
                self.answered = true;
                out.writeln_error(b"pkg: the installer's answer was lost");
                JobProgress::Done(1)
            }
        }
    }
}

impl Drop for PkgJob {
    /// Ctrl+C ends the job without its answer: stop waiting for it, so the
    /// worker's answer is discarded when it comes rather than kept.
    fn drop(&mut self) {
        if !self.answered {
            ANSWER.stop_waiting();
        }
    }
}

fn refuse(state: &mut State) -> Prepared {
    state
        .scrollback
        .push_error(b"pkg: the last request is still with the installer; try again shortly");
    state.last_status = 1;
    Prepared::Refused
}

/// No worker could be started (its stack not mapped, or the kernel refused
/// the thread). The request runs on the window thread instead, as it did
/// before workers, and the person is told the window will wait.
fn say_inline(state: &mut State, errno: i64) {
    let mut num = [0u8; 24];
    let k = format_u64(errno.unsigned_abs(), &mut num);
    let mut line = said(b"pkg: no worker thread (-", &num[..k]);
    line.extend_from_slice(b"), the window waits for the installer");
    state.scrollback.push_line(&line);
}

fn said(what: &[u8], tail: &[u8]) -> Vec<u8> {
    let mut line = Vec::with_capacity(what.len() + tail.len());
    line.extend_from_slice(what);
    line.extend_from_slice(tail);
    line
}
