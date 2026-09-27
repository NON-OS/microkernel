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

//! Whether an input frame came from the input router. Any IPC-capable process
//! can send to any pid's inbox, so the magic proves nothing; the sender the
//! kernel recorded does.

use core::sync::atomic::{AtomicU32, Ordering};

static ROUTER_PID: AtomicU32 = AtomicU32::new(0);

pub(super) fn from_router(sender: u32) -> bool {
    let known = ROUTER_PID.load(Ordering::Acquire);
    if known != 0 && known == sender {
        return true;
    }
    // Looked up again before refusing, so a restarted router is followed.
    let Some(pid) = nonos_service::owner(b"input_router") else {
        return false;
    };
    ROUTER_PID.store(pid, Ordering::Release);
    pid == sender
}
