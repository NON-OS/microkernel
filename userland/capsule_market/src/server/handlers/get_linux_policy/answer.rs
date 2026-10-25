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

//! The answer OP_GET_LINUX_POLICY gives for a listing and release in the
//! accepted index: the release found as readiness finds it, its signatures,
//! the encoded box, and the package hash init holds it to.

use nonos_market_proto::LinuxPolicy;
use nonos_marketplace_abi::MarketplaceIndex;

use super::encode::encode;
use crate::server::handlers::install_ready::find_release::find_release;

/// The answer for `listing` and `release` in the accepted index, with the
/// package hash of the release it is for; None when there is no such
/// release. `publisher` says whether a release's publisher signature
/// verified, by entry and release.
pub(super) fn answer(
    index: &MarketplaceIndex,
    index_signed: bool,
    mut publisher: impl FnMut(usize, usize) -> bool,
    listing: &str,
    release: &str,
) -> Option<(LinuxPolicy, [u8; 32])> {
    // The resolution readiness and the release lookup use, so all agree on
    // which release an empty id means.
    let (at, release_at, rel) = find_release(index, listing, release)?;
    let entry = index.entries.get(at)?;
    Some((encode(entry, rel, index_signed, publisher(at, release_at)), rel.package_hash))
}
