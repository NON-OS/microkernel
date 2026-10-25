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

//! The device proof's scope for a market listing.
//!
//! e = words_of(BLAKE3("NONOS-MARKET-SCOPE-v1" | listing_id | window LE))[0..2],
//! window the unix day of proving. Only nonos.prove's PROVE op derives it;
//! the interactive prover's scope is a different hash and can never equal
//! it. A device has one tag per scope, so its proofs link within one listing
//! and one day and nowhere wider.

use alloc::vec::Vec;

const MARKET_DOMAIN: &[u8] = b"NONOS-MARKET-SCOPE-v1";
const DELIVERY_DOMAIN: &[u8] = b"NONOS-DELIVERY-SCOPE-v1";

/// The window of a unix time in seconds.
pub fn day(unix_seconds: u64) -> u64 {
    unix_seconds / 86_400
}

/// The two scope words, each reduced below p as the gates read a digest.
pub fn market_scope(listing_id: &[u8; 32], window: u64) -> [u64; 2] {
    scope_in(MARKET_DOMAIN, listing_id, window)
}

/// e2, a delivery's scope: the market scope's shape in its own
/// domain, so a device's delivery tags share nothing with its kind-1 tags.
pub fn delivery_scope(listing_id: &[u8; 32], window: u64) -> [u64; 2] {
    scope_in(DELIVERY_DOMAIN, listing_id, window)
}

fn scope_in(domain: &[u8], listing_id: &[u8; 32], window: u64) -> [u64; 2] {
    let mut b = Vec::with_capacity(domain.len() + 40);
    b.extend_from_slice(domain);
    b.extend_from_slice(listing_id);
    b.extend_from_slice(&window.to_le_bytes());
    let w = nox_verify::attest::digest_words(blake3::hash(&b).as_bytes());
    [w[0], w[1]]
}
