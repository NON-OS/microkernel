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

//! `OP_GET_LINUX_POLICY` (I-C5): the box a Linux release is sold with, for
//! init's install drain to hand the install role.

use alloc::vec::Vec;

use super::super::error::MarketError;
use super::super::protocol::{encode_request, OP_GET_LINUX_POLICY};
use super::linux_policy_wire::{parse, LinuxPolicyReply};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

/// Without the caller gate, as the other queued calls: the install it serves
/// passed `can_app_install` in the syscall that queued it. The hash is the
/// package hash of the release the market answered for.
pub(crate) fn queued_get_linux_policy(
    listing_id: &str,
    release_id: &str,
) -> Result<(LinuxPolicyReply, [u8; 32]), MarketError> {
    let mut body: Vec<u8> = Vec::with_capacity(8 + listing_id.len() + release_id.len());
    body.extend_from_slice(&(listing_id.len() as u32).to_le_bytes());
    body.extend_from_slice(listing_id.as_bytes());
    body.extend_from_slice(&(release_id.len() as u32).to_le_bytes());
    body.extend_from_slice(release_id.as_bytes());

    let request_id = next_request_id();
    let frame = encode_request(OP_GET_LINUX_POLICY, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    parse(&resp.body).ok_or(MarketError::ProtocolMismatch)
}
