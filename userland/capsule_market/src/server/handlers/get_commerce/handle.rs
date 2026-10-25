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

use super::super::install_ready::find_release::find_release;
use super::encode::encode;
use crate::protocol::{Request, E_INVAL, E_MSGSIZE, E_NODATA};
use crate::server::error::reply_status;
use crate::server::lp::parse_pair;
use crate::server::payload::{body_slot, reply_with_body};
use crate::store::Store;

pub(crate) fn handle(store: &Store, body: &[u8], req: &Request, tx: &mut [u8]) {
    let accepted = match store.current() {
        Some(a) => a,
        None => return reply_status(tx, req, E_NODATA),
    };
    let (listing_id, release_id) = match parse_pair(body) {
        Some(p) => p,
        None => return reply_status(tx, req, E_INVAL),
    };
    // The same resolution readiness and the release lookup use, so all three
    // agree on which release an empty id means.
    let (entry_index, release_index, release) =
        match find_release(&accepted.index, listing_id, release_id) {
            Some(p) => p,
            None => return reply_status(tx, req, E_NODATA),
        };
    let Some(entry) = accepted.index.entries.get(entry_index) else {
        return reply_status(tx, req, E_NODATA);
    };
    let signed = accepted.publisher_signature_verified(entry_index, release_index);
    let Some(out) = encode(entry, release, signed) else {
        return reply_status(tx, req, E_MSGSIZE);
    };
    let slot = match body_slot(tx, out.len()) {
        Some(s) => s,
        None => return reply_status(tx, req, E_MSGSIZE),
    };
    slot.copy_from_slice(&out);
    reply_with_body(tx, req, out.len());
}
