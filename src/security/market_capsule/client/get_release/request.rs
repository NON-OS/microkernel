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

//! The `OP_GET_RELEASE` request, gated and as init's install drain sends it.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::super::capability::gate_call;
use super::super::super::error::MarketError;
use super::super::super::protocol::{encode_request, OP_GET_RELEASE};
use super::super::seq::next_request_id;
use super::super::status_map::lift;
use super::super::transport::round_trip;
use super::parse;

pub struct ReleaseSummary {
    pub release_id: String,
    pub manifest_hash: [u8; 32],
    pub package_hash: [u8; 32],
    pub package_url: String,
    pub raw: Vec<u8>,
}

pub fn get_release(listing_id: &str, release_id: &str) -> Result<ReleaseSummary, MarketError> {
    gate_call()?;
    queued_get_release(listing_id, release_id)
}

/// Without the caller gate, for init's install drain: the request it serves
/// passed `can_app_install` in the syscall that queued it, and init is not
/// the caller the gate is about.
pub(crate) fn queued_get_release(
    listing_id: &str,
    release_id: &str,
) -> Result<ReleaseSummary, MarketError> {
    let mut body: Vec<u8> = Vec::with_capacity(8 + listing_id.len() + release_id.len());
    body.extend_from_slice(&(listing_id.len() as u32).to_le_bytes());
    body.extend_from_slice(listing_id.as_bytes());
    body.extend_from_slice(&(release_id.len() as u32).to_le_bytes());
    body.extend_from_slice(release_id.as_bytes());

    let request_id = next_request_id();
    let frame = encode_request(OP_GET_RELEASE, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    parse(resp.body).ok_or(MarketError::ProtocolMismatch)
}
