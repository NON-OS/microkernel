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

//! The network work a refresh does, held between ticks: the diagnostic,
//! the account snapshot, or a reading of the payments still on their way. Each tick steps
//! it once, and only a finished job's result reaches the state. A snapshot
//! remembers the account and network it was asked for, so one that lands
//! after a switch is dropped rather than shown against the wrong account.

use alloc::vec::Vec;

use nonos_route_link::Route;

use super::super::read_snapshot::{parse_snapshot, snapshot_request, Snapshot};
use super::super::status::NetStatus;
use super::exchange::{Ended, Exchange};
use super::probe::Probe;

pub enum Job {
    Probe(Probe),
    Snapshot { exchange: Exchange, address: [u8; 20], chain: u64 },
    Follow { exchange: Exchange, hashes: Vec<[u8; 32]>, address: [u8; 20], chain: u64 },
}

/// What one reading of the chain said of the payments on their way.
pub struct Followed {
    /// Each payment's receipt: its block and whether it succeeded, or None.
    pub receipts: Vec<([u8; 32], Option<(u64, bool)>)>,
    pub head: u64,
    pub latest: u64,
    pub pending: u64,
}

/// A job's result, once it has one.
pub enum Finished {
    Probe(NetStatus),
    /// None when the request did not complete, so the link is down.
    Snapshot {
        snap: Option<Snapshot>,
        address: [u8; 20],
        chain: u64,
        /// The host that answered, or failed to.
        host: &'static str,
    },
    /// None when the reading did not come whole.
    Follow {
        followed: Option<Followed>,
        address: [u8; 20],
        chain: u64,
    },
}

const ID_HEAD: u64 = 90;
const ID_LATEST: u64 = 91;
const ID_PENDING: u64 = 92;
const ID_RECEIPT: u64 = 100;

impl Job {
    pub fn probe() -> Job {
        Job::Probe(Probe::begin())
    }

    pub fn snapshot(address: &[u8; 20]) -> Job {
        let body = snapshot_request(address);
        let exchange = Exchange::begin(Route::for_wallet(), body);
        let chain = crate::wallet::chain::current().id;
        Job::Snapshot { exchange, address: *address, chain }
    }

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

    /// One step; the result once the job is finished.
    pub fn step(&mut self) -> Option<Finished> {
        match self {
            Job::Probe(probe) => probe.step().map(Finished::Probe),
            Job::Snapshot { exchange, address, chain } => {
                let snap = match exchange.step()? {
                    Ended::Answer(resp) => Some(parse_snapshot(&resp)),
                    Ended::Failed(_) | Ended::Unknown(_) => None,
                };
                let host = exchange.host;
                Some(Finished::Snapshot { snap, address: *address, chain: *chain, host })
            }
            Job::Follow { exchange, hashes, address, chain } => {
                let followed = match exchange.step()? {
                    Ended::Answer(resp) => followed(&resp, hashes),
                    Ended::Failed(_) | Ended::Unknown(_) => None,
                };
                Some(Finished::Follow { followed, address: *address, chain: *chain })
            }
        }
    }
}

/* A reading is taken only whole: the head and both nonces, and an answer
 * for each receipt, null or not. A receipt that names another transaction
 * is no receipt of this one. */
fn followed(resp: &[u8], hashes: &[[u8; 32]]) -> Option<Followed> {
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
