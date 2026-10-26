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

//! OP_GET_LOGO: a listing's logo, served only when its bytes are the ones
//! the accepted index signs; the store asks once per listing.

use nonos_libc::mk_debug;
use nonos_market_proto::logo::logo_body;

use crate::logos::{embedded, served, Served};
use crate::protocol::{Request, E_INVAL, E_KEYREJECTED, E_MSGSIZE, E_NODATA};
use crate::server::error::reply_status;
use crate::server::lp::read_lp_str;
use crate::server::payload::{body_slot, reply_with_body};
use crate::store::Store;

pub(crate) fn handle(store: &Store, body: &[u8], req: &Request, tx: &mut [u8]) {
    let Some(accepted) = store.current() else {
        return reply_status(tx, req, E_NODATA);
    };
    let Some(listing_id) = read_lp_str(body) else {
        return reply_status(tx, req, E_INVAL);
    };
    let Some(entry) = accepted.index.entries.iter().find(|e| e.listing_id == listing_id) else {
        return reply_status(tx, req, E_NODATA);
    };
    let out = match served(&entry.logo_hash, embedded(listing_id)) {
        Served::None => logo_body(None),
        Served::Logo(bytes) => logo_body(Some(bytes)),
        Served::Refused => {
            let id = listing_id.as_bytes();
            let line = [b"[MARKET] logo refused for ", id, b": not the signed BLAKE3\n"].concat();
            let _ = mk_debug(line.as_ptr(), line.len());
            return reply_status(tx, req, E_KEYREJECTED);
        }
    };
    let Some(out) = out else {
        return reply_status(tx, req, E_MSGSIZE);
    };
    let Some(slot) = body_slot(tx, out.len()) else {
        return reply_status(tx, req, E_MSGSIZE);
    };
    slot.copy_from_slice(&out);
    reply_with_body(tx, req, out.len());
}
