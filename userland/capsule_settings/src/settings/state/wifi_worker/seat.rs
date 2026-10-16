// NONOS Operating System (AGPL-3.0-or-later)

//! Handing a request to the one worker, and picking up its answer.

use alloc::boxed::Box;

use nonos_libc::{Handoff, Look, WorkerSeat};

use super::ask::{Answer, Ask};
use super::work::work;

/// What became of a request handed to `send`.
pub(in super::super) enum Sent {
    /// A worker has it; a tick will find the answer.
    Out,
    /// The last request is still with the driver; this one was dropped.
    Busy,
    /// No worker could be started (the kernel refused the thread, or its stack
    /// could not be mapped): the request comes back, for
    /// the window thread to run itself.
    Refused(Box<Ask>),
}

pub(super) static ANSWER: Handoff<Answer> = Handoff::new();

static SEAT: WorkerSeat = WorkerSeat::new();

/// The seat's refusal while its last worker is still on it.
const EBUSY: i64 = -16;

/// Hand `ask` to a worker.
pub(in super::super) fn send(ask: Box<Ask>) -> Sent {
    if !ANSWER.claim() {
        return Sent::Busy;
    }
    let arg = Box::into_raw(ask);
    match SEAT.spawn(work, arg as usize) {
        Ok(_) => Sent::Out,
        Err(errno) => {
            // SAFETY: the spawn failed, so no worker took the box; it is
            // still this thread's.
            let ask = unsafe { Box::from_raw(arg) };
            ANSWER.release();
            if errno == EBUSY {
                // The last worker has answered but is not gone yet.
                return Sent::Busy;
            }
            Sent::Refused(ask)
        }
    }
}

/// The answer, once a worker has put it in.
pub(in super::super) fn poll() -> Look<Answer> {
    ANSWER.poll()
}
