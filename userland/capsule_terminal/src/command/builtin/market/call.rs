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

//! One call to `market.index`. The framing and the checks on the reply are
//! `nonos_market_proto`'s: a reply that is not the market's, or answers
//! another call, is never read.

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};
use nonos_market_proto::{reply_body, request, SERVICE};

use super::failure::Failure;

/// The catalogue reply carries every listing, so this is sized for it.
const RX_CAP: usize = 96 << 10;

/// As the Marketplace window waits: long enough for the capsule to walk its
/// index, short enough that a silent market does not hang the prompt.
const TIMEOUT_MS: u64 = 1500;

/// Ids only have to differ between this process's calls.
static NEXT_ID: AtomicU32 = AtomicU32::new(1);

/// The reply's body after its status word.
pub(super) fn call(op: u16, body: &[u8]) -> Result<Vec<u8>, Failure> {
    let port = port().ok_or(Failure::NotAnnounced)?;
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let tx = request(op, id, body);
    let mut rx = vec![0u8; RX_CAP];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    let got = usize::try_from(rc).map_err(|_| Failure::Call(rc))?.min(rx.len());
    reply_body(&rx[..got], op, id).map(<[u8]>::to_vec).map_err(Failure::Reply)
}

/// The port the market announced, if it has.
fn port() -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0 && port != 0).then_some(port)
}
