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

//! The market wire as its clients speak it: which replies are believed,
//! and that each body reads back what the market wrote.

use alloc::vec::Vec;

use nonos_market_proto::{
    listing_body, pair_body, parse_app, parse_list, parse_release, reply_body, request,
    status_name, ReplyError, HDR_LEN, MAGIC, OP_GET_APP, OP_LIST_APPS, VERSION,
};

use crate::readiness_tests::release;
use crate::release_wire::encode;

/// A reply as the market capsule frames one: its header echoing the
/// request, then the status, then the body.
fn reply(op: u16, id: u32, status: i32, body: &[u8]) -> Vec<u8> {
    let mut rx = Vec::new();
    rx.extend_from_slice(&MAGIC.to_le_bytes());
    rx.extend_from_slice(&VERSION.to_le_bytes());
    rx.extend_from_slice(&op.to_le_bytes());
    rx.extend_from_slice(&0u16.to_le_bytes());
    rx.extend_from_slice(&0u16.to_le_bytes());
    rx.extend_from_slice(&id.to_le_bytes());
    rx.extend_from_slice(&(4 + body.len() as u32).to_le_bytes());
    rx.extend_from_slice(&status.to_le_bytes());
    rx.extend_from_slice(body);
    rx
}

fn lp(out: &mut Vec<u8>, s: &[u8]) {
    out.extend_from_slice(&(s.len() as u32).to_le_bytes());
    out.extend_from_slice(s);
}

#[path = "stage_tests.rs"]
mod stage_tests;
#[path = "wire_believed_tests.rs"]
mod wire_believed_tests;
#[path = "wire_bodies_tests.rs"]
mod wire_bodies_tests;
#[path = "wire_catalogue_tests.rs"]
mod wire_catalogue_tests;
#[path = "wire_cut_tests.rs"]
mod wire_cut_tests;
#[path = "wire_listing_tests.rs"]
mod wire_listing_tests;
