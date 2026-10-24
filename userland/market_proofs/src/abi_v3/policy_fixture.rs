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

//! A Linux release with a policy laid out by hand from plan/30.

use nonos_marketplace_abi::{CapsuleRelease, MarketplaceEntry};

use super::fixtures::{free_commerce, sold};

pub(super) fn linux(policy: Vec<u8>, url: &str) -> (MarketplaceEntry, CapsuleRelease) {
    let mut e = sold().entries[0].clone();
    e.listing_id = "linux.pkg".into();
    // Free, so readiness rests on the release alone and not on a bond.
    e.commerce = Some(free_commerce());
    let mut rel = e.releases[0].clone();
    rel.package_url = url.into();
    rel.ext.as_mut().unwrap().guest_policy = policy;
    (e, rel)
}

pub(super) fn policy(listing: [u8; 32], release: [u8; 32]) -> Vec<u8> {
    let mut p = vec![0u8; 256];
    p[..8].copy_from_slice(b"NLXPOL01");
    p[8..10].copy_from_slice(&1u16.to_le_bytes());
    p[56..88].copy_from_slice(&listing);
    p[88..120].copy_from_slice(&release);
    p
}
