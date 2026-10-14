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

//! Following the payments on their way: the batch that asks after them,
//! and reading its answer back.

use alloc::vec::Vec;

use nonos_route_link::Route;

use super::exchange::Exchange;
use super::job::{Followed, Job};

const ID_HEAD: u64 = 90;
const ID_LATEST: u64 = 91;
const ID_PENDING: u64 = 92;
const ID_RECEIPT: u64 = 100;

impl Job {
    /// One batch: the newest block, the account's nonce in it and counting
    /// what waits, and the receipt of each payment in `hashes`.
    pub fn follow(hashes: Vec<[u8; 32]>, address: &[u8; 20]) -> Job {
        use crate::wallet::rpc::{request_block_number, request_nonce, request_nonce_latest};
        let mut parts = alloc::vec![
            request_block_number(ID_HEAD),
            request_nonce_latest(address, ID_LATEST),
            request_nonce(address, ID_PENDING),
        ];
        for (i, h) in hashes.iter().enumerate() {
            parts.push(crate::wallet::rpc::request_receipt(h, ID_RECEIPT + i as u64));
        }
        let refs: Vec<&[u8]> = parts.iter().map(Vec::as_slice).collect();
        let body = crate::wallet::rpc::request_batch(&refs);
        let chain = crate::wallet::chain::current().id;
        let exchange = Exchange::begin(Route::for_wallet(), body);
        Job::Follow { exchange, hashes, address: *address, chain }
    }
}

/* A reading is taken only whole: the head and both nonces, and an answer
 * for each receipt, null or not. A receipt that names another transaction
 * is no receipt of this one. */
pub(super) fn followed(resp: &[u8], hashes: &[[u8; 32]]) -> Option<Followed> {
    use crate::wallet::rpc::{object_for_id, parse_receipt_block, parse_receipt_for, parse_u64};
    let number = |id| object_for_id(resp, id).and_then(parse_u64);
    let (head, latest, pending) = (number(ID_HEAD)?, number(ID_LATEST)?, number(ID_PENDING)?);
    let mut receipts = Vec::with_capacity(hashes.len());
    for (i, h) in hashes.iter().enumerate() {
        let obj = object_for_id(resp, ID_RECEIPT + i as u64)?;
        if obj.windows(7).any(|w| w == b"\"error\"") {
            return None;
        }
        let found = parse_receipt_for(obj, h).zip(parse_receipt_block(obj)).map(|(ok, b)| (b, ok));
        receipts.push((*h, found));
    }
    Some(Followed { receipts, head, latest, pending })
}
