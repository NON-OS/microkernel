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

//! Waiting for the transport net.nym sits on.

use nonos_libc::mk_ipc_recv;

use crate::setup;

const OWN_INBOX: u64 = 0;
const RETRY_BACKOFF_MS: u64 = 250;

// The transport this capsule sits on is absent in every profile that does
// not build it, so this wait has no bound. Retrying through `mk_yield` left
// the capsule permanently runnable, which kept the scheduler off its halt
// path and burned a whole core for the life of the boot. A timed receive
// parks it off the run queue instead; no client can be served until setup
// succeeds, so a request landing inside the window is no more lost than it
// is today.
pub fn wait_for_setup() {
    let mut idle = [0u8; 1];
    loop {
        if setup::run().is_ok() {
            return;
        }
        let _ = mk_ipc_recv(OWN_INBOX, idle.as_mut_ptr(), idle.len(), RETRY_BACKOFF_MS);
    }
}
