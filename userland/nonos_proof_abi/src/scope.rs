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
//! The verifier id is "nonos.market:" followed by the listing id's 32 bytes,
//! and the window is the unix day. A device has one tag per scope, so its
//! records link within one listing and one day and nowhere wider.

use nonos_device_attest::scope as device_scope;

pub const MARKET_PREFIX: &[u8; 13] = b"nonos.market:";

const ID_BYTES: usize = MARKET_PREFIX.len() + 32;

pub fn verifier_id(listing_id: &[u8; 32]) -> [u8; ID_BYTES] {
    let mut id = [0u8; ID_BYTES];
    id[..MARKET_PREFIX.len()].copy_from_slice(MARKET_PREFIX);
    id[MARKET_PREFIX.len()..].copy_from_slice(listing_id);
    id
}

/// The window of a unix time in seconds.
pub fn day(unix_seconds: u64) -> u64 {
    unix_seconds / 86_400
}

/// The two scope words, exactly as `nonos_device_attest::scope` derives them,
/// so a prover and a verifier cannot pack the same listing differently.
pub fn scope(listing_id: &[u8; 32], day: u64) -> Option<[u64; 2]> {
    let e = device_scope(&verifier_id(listing_id), day)?;
    Some([e[0].to_u64(), e[1].to_u64()])
}
