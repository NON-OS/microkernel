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

//! One turn of the serve loop. Every receive waits at most `WAIT_MS` (C7):
//! one that ends with nothing returns, and the next waits again; one that
//! fails without waiting parks in `recv_ready`. Either way the loop holds
//! no core between requests.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, recv_ready};

use super::answer::answer;
use super::wire::{Reply, RX_LEN};
use crate::setup::Driver;

const SERVICE_INBOX: u64 = 0;
pub const WAIT_MS: u64 = 1_000;

/// One receive and, when a process sent something, its reply. A message
/// from the kernel (sender 0) has nobody to answer and is dropped.
pub fn turn(d: &mut Driver, rx: &mut [u8; RX_LEN], tx: &mut Reply) {
    let mut sender = 0u32;
    let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), WAIT_MS, &mut sender);
    if !recv_ready(n) || sender == 0 {
        return;
    }
    // The kernel copies at most the buffer; a longer message arrives cut.
    let got = usize::try_from(n).map_or(RX_LEN, |n| n.min(RX_LEN));
    let len = answer(d, sender, &rx[..got], tx);
    let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
}
