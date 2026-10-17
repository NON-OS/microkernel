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

//! The ids the market answers with for a store's names: the
//! listing's of the publisher's key and its text, the release's of the
//! listing's and its text, hashed with keccak on the host.

use nonos_market_proto::Commerce;
use nonos_marketplace_abi::{listing_id, release_id, SettleMode};
use nonos_payment_proto::Names;

use super::deposit_fixture::K;
use super::fixture::buy;
use super::gate_fixture::names;
use super::read_fixture::commerce;
use crate::order::buy::Buy;

/// The publisher's key, made up.
pub const KEY: [u8; 32] = [0x2B; 32];
/// The listing's name as the market shows it, never the store's.
pub const TITLE: &[u8] = b"Quiet Notes";
/// The release the market resolves an empty name to.
pub const DEFAULT: &[u8] = b"linux.quiet-notes@1.4.0";

fn text(b: &[u8]) -> &str {
    core::str::from_utf8(b).unwrap()
}

/// The release's id text the market answers `n` with.
pub fn version(n: &Names) -> Vec<u8> {
    match n.release.is_empty() {
        true => DEFAULT.to_vec(),
        false => n.release.clone(),
    }
}

/// listingId and releaseId for `n`.
pub fn ids(n: &Names) -> ([u8; 32], [u8; 32]) {
    let l = listing_id(&K, &KEY, text(&n.listing));
    (l, release_id(&K, &l, text(&version(n))))
}

/// GET_COMMERCE's answer for `n`.
pub fn commerce_for(n: &Names, mode: SettleMode) -> Commerce {
    let (listing_id, release_id) = ids(n);
    Commerce { listing_id, release_id, ..commerce(mode) }
}

/// The fixture's purchase, as the market's ids for names() make it.
pub fn bought() -> Buy {
    let (listing_id, release_id) = ids(&names());
    Buy { listing_id, release_id, ..buy() }
}
