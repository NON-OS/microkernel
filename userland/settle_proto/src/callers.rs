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

//! Who may call which settle op. Every argument reaches an
//! RPC host or a URL, so a capsule without Network must never reach the
//! network through settle. Decided by which registry names the kernel-stamped
//! sender pid owns, as the meter decides; pure, the capsule supplies
//! `owner_of`.

use crate::ops::{OP_BROADCAST, OP_CHANNEL_STATE, OP_FETCH, OP_GAS, OP_HEALTHCHECK};
use crate::ops::{OP_DELIVERED, OP_INTENT, OP_LISTING, OP_OPENED, OP_READ, OP_SETTLE};

const W0: &[u8] = b"app.nonos_wallet";
const W1: &[u8] = b"app.nonos_wallet.1";
const W2: &[u8] = b"app.nonos_wallet.2";

pub const BROADCASTERS: [&[u8]; 4] = [b"payment", W0, W1, W2];
pub const READERS: [&[u8]; 7] =
    [b"payment", W0, W1, W2, b"app.store", b"market.index", b"installer"];
pub const FETCHERS: [&[u8]; 2] = [b"app.store", b"market.index"];
/// FETCH 1 ESCROW, and the PAGEs of its answer (v1.4.24).
pub const ESCROW_FETCHERS: [&[u8]; 1] = [b"payment"];
const ALL_FETCHERS: [&[u8]; 3] = [b"app.store", b"market.index", b"payment"];
pub const SETTLERS: [&[u8]; 4] = [W0, W1, W2, b"installer"];

/// The names that may call `op`: None for anyone (HEALTHCHECK), an empty
/// list for an op settle does not answer.
pub fn callers(op: u16) -> Option<&'static [&'static [u8]]> {
    match op {
        OP_HEALTHCHECK => None,
        OP_BROADCAST => Some(&BROADCASTERS),
        OP_CHANNEL_STATE | OP_GAS | OP_OPENED | OP_READ | OP_INTENT => Some(&READERS),
        // Settle checks each FETCH form's own list (fetch_callers).
        OP_FETCH => Some(&ALL_FETCHERS),
        OP_LISTING => Some(&FETCHERS),
        OP_SETTLE => Some(&SETTLERS),
        // DELIVERED (v1.4.46): the meter alone reads an order's K.
        OP_DELIVERED => Some(&ESCROW_FETCHERS),
        _ => Some(&[]),
    }
}

/// Whether `sender` may call `op`, given the pid owning each name. Pid zero
/// is no sender at all.
pub fn admits(op: u16, sender: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> bool {
    match callers(op) {
        None => sender != 0,
        Some(names) => sender != 0 && names.iter().any(|n| owner_of(n) == Some(sender)),
    }
}

/// Who may send this FETCH body: ESCROW payment alone, RELEASE the store and
/// the market. A PAGE goes to whoever settle holds that key for.
pub fn fetch_callers(body: &[u8]) -> &'static [&'static [u8]] {
    match body.first() {
        Some(&crate::fetch::FETCH_ESCROW) => &ESCROW_FETCHERS,
        Some(&crate::fetch::FETCH_RELEASE) => &FETCHERS,
        _ => &ALL_FETCHERS,
    }
}
