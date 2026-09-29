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

//! Answering a caller that arrives before the transport is up.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};

use crate::protocol::{E_NO_TCP, HDR_LEN, MAGIC, VERSION};

const OWN_INBOX: u64 = 0;
const RETRY_BACKOFF_MS: u64 = 250;

/*
 * The transport underneath is absent in every profile that does not build it, so
 * this wait has no bound. An early request is answered rather than dequeued and
 * dropped: the mixnet transport read early requests into a one byte buffer,
 * destroying them, and the first page load after every boot ate its whole timeout
 * waiting for a reply that had already been thrown away.
 *
 * A timed receive rather than a yield loop, so the capsule parks off the run
 * queue instead of keeping a core busy for the life of the boot.
 */
pub fn wait_for_setup() {
    let mut rx = [0u8; HDR_LEN + 64];
    loop {
        if crate::setup::run().is_ok() {
            return;
        }
        let mut sender = 0u32;
        let n =
            mk_ipc_recv_from(OWN_INBOX, rx.as_mut_ptr(), rx.len(), RETRY_BACKOFF_MS, &mut sender);
        if n > 0 && sender != 0 {
            refuse(sender, &rx);
        }
    }
}

fn refuse(sender: u32, rx: &[u8]) {
    let mut tx = [0u8; HDR_LEN];
    let op = u16::from_le_bytes([rx[6], rx[7]]);
    let id = u32::from_le_bytes([rx[12], rx[13], rx[14], rx[15]]);
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[8..10].copy_from_slice(&E_NO_TCP.to_le_bytes());
    tx[12..16].copy_from_slice(&id.to_le_bytes());
    let _ = mk_ipc_reply(sender, tx.as_ptr(), HDR_LEN);
}
