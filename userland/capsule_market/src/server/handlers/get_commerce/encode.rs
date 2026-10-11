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

//! The `OP_GET_COMMERCE` body, laid out as `nonos_market_proto::commerce`
//! reads it.

extern crate alloc;

use alloc::vec::Vec;

use nonos_marketplace_abi::MarketplaceEntry;
use nonos_marketplace_abi::{binding_verdict, commerce_bytes, commerce_verdict, listing_id};
use nonos_marketplace_abi::{release_hash, release_id, CapsuleRelease, CommerceVerdict};

use crate::chain_hosts::Host;

fn lp(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(bytes);
}

/// `publisher_signed` is whether this release's publisher signature
/// verified: a commerce hash nobody signed is not the publisher's word.
pub(super) fn encode(
    entry: &MarketplaceEntry,
    rel: &CapsuleRelease,
    publisher_signed: bool,
) -> Vec<u8> {
    let commerce = match commerce_verdict(&Host, entry, rel) {
        CommerceVerdict::Matches if !publisher_signed => CommerceVerdict::Mismatch,
        v => v,
    };
    let listing = listing_id(&Host, &entry.publisher_pubkey, &entry.listing_id);
    let mut out = Vec::with_capacity(256);
    out.push(commerce as u8);
    out.push(binding_verdict(&Host, &Host, entry) as u8);
    out.extend_from_slice(&listing);
    out.extend_from_slice(&release_id(&Host, &listing, &rel.release_id));
    out.extend_from_slice(&release_hash(&Host, rel).unwrap_or([0; 32]));
    out.extend_from_slice(&entry.token.chain_id.to_le_bytes());
    lp(&mut out, &entry.token.contract_address);
    match &entry.commerce {
        Some(c) => lp(&mut out, &commerce_bytes(c)),
        None => lp(&mut out, &[]),
    }
    out
}
