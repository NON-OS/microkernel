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

//! The on-chain ids a publisher needs to publish this release to the
//! listing registry (plan/22, plan/23), printed beside its signature.

use nonos_marketplace_abi::{listing_id, release_hash, release_id, CapsuleRelease};

use crate::hosts::Host;

pub(super) fn print_ids(app_id: &str, publisher: &[u8; 32], release: &CapsuleRelease) {
    let listing = listing_id(&Host, publisher, app_id);
    println!("listing_id 0x{}", hex::encode(listing));
    println!("release_id 0x{}", hex::encode(release_id(&Host, &listing, &release.release_id)));
    if let Some(hash) = release_hash(&Host, release) {
        println!("release_hash 0x{}", hex::encode(hash));
    }
}
