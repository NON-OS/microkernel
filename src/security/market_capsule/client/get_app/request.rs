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

//! The `OP_GET_APP` request and the summary its reply lifts into.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::super::capability::gate_call;
use super::super::super::error::MarketError;
use super::super::super::protocol::{encode_request, OP_GET_APP};
use super::super::seq::next_request_id;
use super::super::status_map::lift;
use super::super::transport::round_trip;
use super::parse;

pub struct AppSummary {
    pub listing_id: String,
    pub capsule_id: [u8; 32],
    pub name: String,
    pub publisher_name: String,
    pub publisher_pubkey: [u8; 32],
    pub description: String,
    pub release_count: u32,
}

pub fn get_app(listing_id: &str) -> Result<AppSummary, MarketError> {
    let _caller = gate_call()?;
    let mut body: Vec<u8> = Vec::with_capacity(4 + listing_id.len());
    body.extend_from_slice(&(listing_id.len() as u32).to_le_bytes());
    body.extend_from_slice(listing_id.as_bytes());

    let request_id = next_request_id();
    let frame = encode_request(OP_GET_APP, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    parse(&resp.body).ok_or(MarketError::ProtocolMismatch)
}
