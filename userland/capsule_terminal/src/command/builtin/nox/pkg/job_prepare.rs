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

//! A `pkg` request handed to a worker thread, or back to the window thread when none can take it.

use alloc::boxed::Box;

use super::super::work::{parse, perform, PkgOp};
use super::job_say::{refuse, said, say_inline};
use super::{PkgJob, Prepared, ANSWER, SEAT};
use crate::term::state::State;

/// The seat's refusal while its last worker is still on it.
const EBUSY: i64 = -16;

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
