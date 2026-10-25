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

//! One NBTC request answered, and its reply sent to the caller.

use core::cell::RefCell;

use super::kept::Kept;
use crate::nbtc::wire::REPLY_MAX;
use crate::nbtc::{answer, busy, caller, Board};
use nonos_libc::{mk_ipc_reply, mk_service_lookup, mk_uptime_ms};

pub(super) fn serve(board: &RefCell<Board<Kept>>, request: &[u8], from: u32) {
    let who = caller(from, owner_of);
    let mut tx = [0u8; REPLY_MAX];
    let n = match board.try_borrow_mut() {
        Ok(mut b) => answer(&mut b, who, request, now(), &mut tx),
        Err(_) => busy(request, &mut tx),
    };
    // A sender that called nothing has no reply waiting; the kernel finds
    // none to pair this with and the reply goes nowhere, which is right.
    let _ = mk_ipc_reply(from, tx.as_ptr(), n);
}

pub(super) fn now() -> u64 {
    mk_uptime_ms().max(0) as u64
}

/// The pid that holds `name` now, from the registry.
fn owner_of(name: &[u8]) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc == 0 && pid != 0).then_some(pid)
}
