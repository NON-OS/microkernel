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

/*
 * A release's withdraw and anchors, from the entry: the listingId and
 * releaseId as marketplace_abi derives them, and one kind 1 anchor per
 * proof_anchors word (Q5).
 */

use alloc::vec::Vec;

use nonos_marketplace_abi::{listing_id, release_id, Keccak};
use nonos_marketplace_abi::{CapsuleRelease, MarketplaceEntry};

use super::super::publisher::Refused;
use super::call::Publisher;
use super::entry::Calls;
use super::why::{ours, Why};

/// Once the entry is ours: its bond id, release `i`, the listingId and
/// that release's releaseId.
pub(super) struct Listed<'e> {
    pub bond_id: [u8; 32],
    pub rel: &'e CapsuleRelease,
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
}

pub(super) fn listed<'e>(
    k: &impl Keccak,
    e: &'e MarketplaceEntry,
    me: &[u8; 20],
    i: usize,
) -> Result<Listed<'e>, Why> {
    let bond_id = ours(k, e, me)?.bond_id;
    let rel = e.releases.get(i).ok_or(Why::Entry(Refused::NoRelease))?;
    let listing_id = listing_id(k, &e.publisher_pubkey, &e.listing_id);
    let release_id = release_id(k, &listing_id, &rel.release_id);
    Ok(Listed { bond_id, rel, listing_id, release_id })
}

pub fn withdraw_of(k: &impl Keccak, e: &MarketplaceEntry, me: &[u8; 20], i: usize) -> Calls {
    let Listed { listing_id, release_id, .. } = listed(k, e, me, i)?;
    Ok(Vec::from([Publisher::WithdrawRelease { listing_id, release_id }]))
}

/// Refused for a schema 2 release, which has no proof_anchors.
pub fn anchors_of(k: &impl Keccak, e: &MarketplaceEntry, me: &[u8; 20], i: usize) -> Calls {
    let Listed { rel, listing_id, .. } = listed(k, e, me, i)?;
    let x = rel.ext.as_ref().ok_or(Why::Schema2)?;
    let anchor = |a: &[u8; 32]| Publisher::Anchor { anchor: *a, kind: 1, listing_id };
    Ok(x.proof_anchors.iter().map(anchor).collect())
}
