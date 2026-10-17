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
 * Whether the market's answers are for the names the store asked (B8).
 * The names are market ids as text: the listing's ("linux.qwenchat") and
 * the release's ("linux.qwenchat@1.2.0", empty for the default). The
 * answers carry the on-chain ids, hashes of those texts:
 * listingId of the publisher's key and the listing's text, releaseId of
 * listingId and the release's text. So GET_APP must echo the listing's
 * text, GET_RELEASE the release's (unless the default was asked), and
 * GET_COMMERCE's ids must be those two hashes of them.
 */

use alloc::vec::Vec;

use nonos_market_proto::{AppIds, Commerce};
use nonos_marketplace_abi::{listing_id, release_id, Keccak};
use nonos_payment_proto::Names;

pub const OTHER_LISTING: &str = "The market answered for another listing than the one the \
     store named, so nothing here can be bought.";
pub const OTHER_RELEASE: &str = "The market answered for another release than the one the \
     store named, so nothing here can be bought.";
pub const UNHASHED: &str = "The wallet could not hash the market's ids itself, so nothing \
     here can be bought.";

/// What is bought, by the market's own names for it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Record {
    /// The listing's name, as the market shows it.
    pub title: Vec<u8>,
    /// The release's id text, as the market resolved it.
    pub release: Vec<u8>,
}

/// The market's names for what `names` asked, once GET_APP's `app`,
/// GET_COMMERCE's `c` and GET_RELEASE's `version` are all for them.
pub fn as_asked(
    k: &impl Keccak,
    names: &Names,
    app: &AppIds,
    c: &Commerce,
    version: &[u8],
) -> Result<Record, &'static str> {
    let text = core::str::from_utf8(&names.listing).map_err(|_| OTHER_LISTING)?;
    // A hash the service did not give is all zeros: never an id to match.
    let listing = hashed(listing_id(k, &app.publisher_key, text))?;
    if app.id != names.listing || c.listing_id != listing {
        return Err(OTHER_LISTING);
    }
    let named = names.release.is_empty() || names.release == version;
    let version_text = core::str::from_utf8(version).map_err(|_| OTHER_RELEASE)?;
    if !named || c.release_id != hashed(release_id(k, &listing, version_text))? {
        return Err(OTHER_RELEASE);
    }
    Ok(Record { title: app.name.clone(), release: version.to_vec() })
}

fn hashed(id: [u8; 32]) -> Result<[u8; 32], &'static str> {
    (id != [0; 32]).then_some(id).ok_or(UNHASHED)
}
