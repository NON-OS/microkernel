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

use nonos_market_proto::linux_policy_body;

use super::answer::answer;
use crate::protocol::{Request, E_INVAL, E_MSGSIZE, E_NODATA};
use crate::server::error::reply_status;
use crate::server::lp::parse_pair;
use crate::server::payload::{body_slot, reply_with_body};
use crate::store::Store;

pub(crate) fn handle(store: &Store, body: &[u8], req: &Request, tx: &mut [u8]) {
    let Some(accepted) = store.current() else {
        return reply_status(tx, req, E_NODATA);
    };
    let Some((listing_id, release_id)) = parse_pair(body) else {
        return reply_status(tx, req, E_INVAL);
    };
    let publisher = |entry, release| accepted.publisher_signature_verified(entry, release);
    let found =
        answer(&accepted.index, accepted.signature_verified, publisher, listing_id, release_id);
    let Some((reply, package_hash)) = found else {
        return reply_status(tx, req, E_NODATA);
    };
    let out = linux_policy_body(&reply, &package_hash);
    let Some(slot) = body_slot(tx, out.len()) else {
        return reply_status(tx, req, E_MSGSIZE);
    };
    slot.copy_from_slice(&out);
    reply_with_body(tx, req, out.len());
}
