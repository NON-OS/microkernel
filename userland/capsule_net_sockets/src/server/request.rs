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

//! Taking a request off the service inbox, and the kernel's id for it.

use core::sync::atomic::{AtomicU64, Ordering};
use nonos_libc::ipc::{mk_ipc_recv_req, IpcReqMeta};

const SERVICE_INBOX: u64 = 0;

/// The kernel's id for the request taken last, 0 when none is owed a reply:
/// what a connect answered after its turn answers by (MkIpcReplyTo).
static REQUEST: AtomicU64 = AtomicU64::new(0);

/// Wait up to `wait` ms (0: until one comes) for a request into `rx`; its
/// sender goes to `sender`, 0 when the kernel sent it.
pub fn receive(rx: &mut [u8], wait: u64, sender: &mut u32) -> i64 {
    let mut meta = IpcReqMeta::default();
    let n = mk_ipc_recv_req(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut meta);
    *sender = meta.sender;
    REQUEST.store(if n > 0 { meta.request } else { 0 }, Ordering::Relaxed);
    n
}

/// The id of the request taken last.
pub fn current() -> u64 {
    REQUEST.load(Ordering::Relaxed)
}
