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

//! The service loop: wait for a guest to make a call the kernel refuses,
//! answer it, hand the answer back. Everything a hosted program can do to
//! this system passes through these few lines.

use nonos_libc::{mk_foreign_reply, mk_foreign_wait, ForeignFrame};

use super::dispatch::answer;
use crate::linux::guest::Guest;

/// How long one wait blocks before looking at its guests again. A guest
/// that exits while nothing is in flight is noticed on the next pass.
const WAIT_MS: u64 = 250;

/// Serve `guest` until it exits, and report its exit code.
pub fn serve(guest: &mut Guest) -> i32 {
    loop {
        let mut frame = ForeignFrame::default();
        let got = mk_foreign_wait(&mut frame, WAIT_MS);
        if got > 0 && frame.pid == guest.pid {
            let value = answer(guest, &frame);
            let _ = mk_foreign_reply(guest.pid, value);
        }
        if let Some(code) = guest.exited {
            return code;
        }
    }
}
