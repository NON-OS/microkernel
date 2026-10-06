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

//! Reading requester addresses out of the validator's node descriptions.

use alloc::vec::Vec;

use super::api::field::string_field;
use super::api::{node_objects, objects};
use super::exit_address::{parse_address, ExitAddress};
use crate::json::find_key;

/// More nodes than the network has, so a hostile answer cannot make the
/// capsule walk without bound.
const MAX_NODES: usize = 8192;

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
