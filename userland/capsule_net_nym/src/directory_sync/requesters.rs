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

//! The network requesters a route can end at, from the validator.
//!
//! A requester is the service on an exit that opens connections on the
//! client's behalf, and its address says who receives what leaves the
//! mixnet. That address used to be asked of each exit over plain HTTP on its
//! node port, and nothing checked the answer: anyone on the path to one exit
//! could name a requester of their own and become the client's exit. The
//! validator publishes every node's self-description over the same TLS
//! connection the node list comes from, so the address is taken from there,
//! and one is kept only when the gateway it names is an exit gateway in the
//! authenticated node list.

use alloc::vec::Vec;

use spin::Mutex;

use super::described::parse_described;
use super::exit_address::ExitAddress;
use super::https::fetch_tls;
use super::live::{API_ADDRESSES, API_HOST};

/// The validator's node self-descriptions, every node in one answer.
const DESCRIBED_PATH: &str = "/api/v1/nym-nodes/described";
/// The whole document ran to 1,563,754 bytes for 841 nodes on 2026-09-28;
/// this leaves room for the network to grow without letting an answer take
/// the heap.
const MAX_DESCRIBED: usize = 4 * 1024 * 1024;
static REQUESTERS: Mutex<Vec<ExitAddress>> = Mutex::new(Vec::new());

/// Fetch the described nodes and keep the requesters on known exits.
/// Returns how many were kept; the cache is replaced only by a non-empty
/// answer, so a bad fetch never empties a working list.
pub fn refresh(tcp_port: u32, is_exit: impl Fn(&[u8; 32]) -> bool) -> Result<usize, u16> {
    let body = fetch_tls(tcp_port, API_HOST, API_ADDRESSES, DESCRIBED_PATH, MAX_DESCRIBED)?;
    let found = parse_described(&body, is_exit);
    if found.is_empty() {
        return Err(13);
    }
    let n = found.len();
    *REQUESTERS.lock() = found;
    Ok(n)
}

/// The cached requesters, in validator order.
pub fn cached() -> Vec<ExitAddress> {
    REQUESTERS.lock().clone()
}
