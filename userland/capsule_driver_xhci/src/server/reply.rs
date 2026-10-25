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

//! Reply routing for the request currently being served. The server
//! loop is strictly sequential: one `recv_from`, one dispatch, one
//! reply. So the sender pid captured before dispatch is the correct
//! destination for every send the handlers emit. A capsule caller
//! (pid != 0) gets a correlation-matched `mk_ipc_reply` delivered to
//! its own reply inbox; a kernel-internal caller (pid 0) keeps the
//! legacy fixed reply endpoint it drains by request id.

use core::sync::atomic::Ordering;

use nonos_libc::{mk_ipc_reply, mk_ipc_send};

use super::served::served;
use crate::protocol::KERNEL_REPLY_ENDPOINT;

pub use super::vouch::{vouch, vouched};

pub fn set_sender(pid: u32) {
    served(|s| s.sender.store(pid, Ordering::Relaxed));
    super::vouch::unvouch();
}

/// The pid of the client whose request is being served, 0 for the kernel.
pub fn sender() -> u32 {
    served(|s| s.sender.load(Ordering::Relaxed))
}

/// While set, a handler's reply is held in its buffer rather than sent, so
/// the controller multiplexer can read it, rewrite the ids in it, and send
/// it itself (`mux`). Run `f` so; returns the length of the reply it built,
/// 0 if it sent none.
pub fn captured(f: impl FnOnce()) -> usize {
    served(|s| {
        s.captured_len.store(0, Ordering::Relaxed);
        s.capture.store(true, Ordering::Relaxed);
    });
    f();
    served(|s| {
        s.capture.store(false, Ordering::Relaxed);
        s.captured_len.load(Ordering::Relaxed)
    })
}

pub fn send(tx: *const u8, len: usize) {
    if served(|s| s.capture.load(Ordering::Relaxed)) {
        served(|s| s.captured_len.store(len, Ordering::Relaxed));
        return;
    }
    let pid = sender();
    if pid != 0 {
        let _ = mk_ipc_reply(pid, tx, len);
    } else {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx, len);
    }
}
