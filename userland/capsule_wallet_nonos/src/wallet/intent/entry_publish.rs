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
 * Q4, the registry's publish of one release of the entry: ids from
 * marketplace_abi, appIdHash keccak of the app id so the registry's
 * recomputed listingId is the entry's, and the uri typed for
 * this release: https, at most MAX_URL, no user, password or port
 * (https_target takes only a DNS name before the path).
 */

use alloc::vec::Vec;

use nonos_marketplace_abi::{release_hash, Keccak, MarketplaceEntry};
use nonos_settle_proto::{https_target, MAX_URL};

use super::call::Publisher;
use super::entry::Calls;
use super::entry_release::{listed, Listed};
use super::why::Why;

pub fn publish_of(
    k: &impl Keccak,
    e: &MarketplaceEntry,
    me: &[u8; 20],
    i: usize,
    uri: &str,
) -> Calls {
    let Listed { bond_id, rel, release_id, .. } = listed(k, e, me, i)?;
    let release_hash = release_hash(k, rel).ok_or(Why::Schema2)?;
    let uri = uri.as_bytes();
    if uri.len() > MAX_URL || https_target(uri).is_none() {
        return Err(Why::Uri);
    }
    Ok(Vec::from([Publisher::Publish {
        publisher_key: e.publisher_pubkey,
        app_id_hash: k.keccak256(e.listing_id.as_bytes()),
        release_id,
        release_hash,
        bond_id,
        uri: uri.to_vec(),
    }]))
}
