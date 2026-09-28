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

use super::api::base58::decode32;
use super::api::field::string_field;
use super::api::{node_objects, objects};
use super::https::fetch_tls;
use crate::json::find_key;

/// The validator's node self-descriptions, every node in one answer.
const DESCRIBED_PATH: &str = "/api/v1/nym-nodes/described";
/// The whole document ran to 1,563,754 bytes for 841 nodes on 2026-09-28;
/// this leaves room for the network to grow without letting an answer take
/// the heap.
const MAX_DESCRIBED: usize = 4 * 1024 * 1024;
/// More nodes than the network has, so a hostile answer cannot make the
/// capsule walk without bound.
const MAX_NODES: usize = 8192;

/// The exit that opens connections on our behalf.
#[derive(Clone, Copy)]
pub struct ExitAddress {
    pub identity: [u8; 32],
    pub encryption: [u8; 32],
    pub gateway: [u8; 32],
}

static REQUESTERS: Mutex<Vec<ExitAddress>> = Mutex::new(Vec::new());

/// Parse `identity.encryption@gateway`, each part a base58 key of 32 bytes.
pub fn parse_address(text: &[u8]) -> Option<ExitAddress> {
    let at = text.iter().position(|&b| b == b'@')?;
    let (client, gateway) = (&text[..at], &text[at + 1..]);
    let dot = client.iter().position(|&b| b == b'.')?;
    Some(ExitAddress {
        identity: decode32(&client[..dot])?,
        encryption: decode32(&client[dot + 1..])?,
        gateway: decode32(gateway)?,
    })
}

/// The first object value under `key` at the top level of `obj`.
fn object_at<'a>(obj: &'a [u8], key: &str) -> Option<&'a [u8]> {
    let at = find_key(obj, key)?;
    objects(&obj[at..], 1).into_iter().next()
}

/// Every requester address in a described answer whose gateway `is_exit`
/// accepts. The gateway is part of the address itself, so an address that
/// names a gateway outside the authenticated exit list is dropped here.
pub fn parse_described(body: &[u8], is_exit: impl Fn(&[u8; 32]) -> bool) -> Vec<ExitAddress> {
    let mut out = Vec::new();
    for node in node_objects(body, MAX_NODES) {
        let Some(requester) =
            object_at(node, "description").and_then(|d| object_at(d, "network_requester"))
        else {
            continue;
        };
        let Some(address) = string_field(requester, "address") else { continue };
        if let Some(exit) = parse_address(&address) {
            if is_exit(&exit.gateway) {
                out.push(exit);
            }
        }
    }
    out
}

/// Fetch the described nodes and keep the requesters on known exits.
/// Returns how many were kept; the cache is replaced only by a non-empty
/// answer, so a bad fetch never empties a working list.
pub fn refresh(tcp_port: u32, is_exit: impl Fn(&[u8; 32]) -> bool) -> Result<usize, u16> {
    let body = fetch_tls(tcp_port, super::live::API_HOST, DESCRIBED_PATH, MAX_DESCRIBED)?;
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
