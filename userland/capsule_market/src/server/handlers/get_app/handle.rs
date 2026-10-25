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

extern crate alloc;

use alloc::vec::Vec;

use nonos_marketplace_abi::MarketplaceEntry;

use crate::protocol::{Request, E_INVAL, E_MSGSIZE, E_NODATA};
use crate::server::error::reply_status;
use crate::server::lp::{read_lp_str, write_count, write_lp};
use crate::server::payload::{body_slot, reply_with_body};
use crate::store::Store;

pub(crate) fn handle(store: &Store, body: &[u8], req: &Request, tx: &mut [u8]) {
    let accepted = match store.current() {
        Some(a) => a,
        None => return reply_status(tx, req, E_NODATA),
    };
    let listing_id = match read_lp_str(body) {
        Some(s) => s,
        None => return reply_status(tx, req, E_INVAL),
    };
    let entry = match accepted.index.entries.iter().find(|e| e.listing_id == listing_id) {
        Some(e) => e,
        None => return reply_status(tx, req, E_NODATA),
    };
    let Some(out) = encode(entry) else {
        return reply_status(tx, req, E_MSGSIZE);
    };
    let body_len = out.len();
    let slot = match body_slot(tx, body_len) {
        Some(s) => s,
        None => return reply_status(tx, req, E_MSGSIZE),
    };
    slot.copy_from_slice(&out);
    reply_with_body(tx, req, body_len);
}

/// None when a field is too long for the u32 length the wire gives it.
fn encode(entry: &MarketplaceEntry) -> Option<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    write_lp(&mut out, entry.listing_id.as_bytes())?;
    out.extend_from_slice(&entry.capsule_id);
    write_lp(&mut out, entry.name.as_bytes())?;
    write_lp(&mut out, entry.publisher_name.as_bytes())?;
    out.extend_from_slice(&entry.publisher_pubkey);
    write_lp(&mut out, entry.description.as_bytes())?;
    write_count(&mut out, entry.releases.len())?;
    Some(out)
}
