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

use super::constants::READINESS_LEN;
use super::find_release::find_release;
use super::parse_ready::parse_ready;
use crate::bonds::Bonds;
use crate::chain_hosts::Host;
use crate::clock::now_ms;
use crate::install_ready::{bond_gate, escrow, evaluate, market_gates, ready};
use crate::protocol::{Request, E_INVAL, E_NODATA};
use crate::server::error::reply_status;
use crate::server::payload::{body_slot, reply_with_body};
use crate::store::Store;

pub(crate) fn handle(store: &Store, bonds: &mut Bonds, body: &[u8], req: &Request, tx: &mut [u8]) {
    let accepted = match store.current() {
        Some(a) => a,
        None => return reply_status(tx, req, E_NODATA),
    };
    let (listing_id, release_id, chain) = match parse_ready(body) {
        Some(p) => p,
        None => return reply_status(tx, req, E_INVAL),
    };
    let (entry_index, release_index, release) =
        match find_release(&accepted.index, listing_id, release_id) {
            Some(p) => p,
            None => return reply_status(tx, req, E_NODATA),
        };
    let publisher_ok = accepted.publisher_signature_verified(entry_index, release_index);
    let Some(entry) = accepted.index.entries.get(entry_index) else {
        return reply_status(tx, req, E_NODATA);
    };
    let base =
        evaluate(accepted.signature_verified, listing_id, release, publisher_ok, escrow(entry));
    let more = market_gates(&Host, entry, release, publisher_ok);
    let code = bond_gate(&mut bonds.cache, entry, chain, now_ms() / 1000);
    let bond = code.passes();
    /* The row's layout, typed to READINESS_LEN, so the build fails if the
     * proto's length and this list ever disagree. */
    let row: [u8; READINESS_LEN] = [
        (ready(accepted.signature_verified, entry, release, publisher_ok) && bond) as u8,
        base.index_signature_valid as u8,
        base.package_url_present as u8,
        base.publisher_signature_present as u8,
        base.validation_passed as u8,
        base.arch_match as u8,
        base.attestation_present as u8,
        more.commerce as u8,
        more.attest as u8,
        more.policy as u8,
        more.price as u8,
        bond as u8,
        code as u8,
    ];
    let Some(slot) = body_slot(tx, READINESS_LEN) else {
        return reply_status(tx, req, E_INVAL);
    };
    slot.copy_from_slice(&row);
    reply_with_body(tx, req, READINESS_LEN);
}
