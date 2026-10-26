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

//! Whether a Linux release's guest policy is the one it was sold under
//! (plan/25). Until the owner merges the tracks (I-B2), nonos_linux_policy is
//! C's crate and is not linked here: this reads only the header and the two
//! id fields, at offsets pinned by tests against plan/30's layout.

use super::policy_kind::PolicyVerdict;
use crate::ids::{listing_id, release_id, Keccak};
use crate::types::{CapsuleRelease, MarketplaceEntry};

pub const POLICY_LEN: usize = 256;
pub const POLICY_MAGIC: [u8; 8] = *b"NLXPOL01";
pub const POLICY_VERSION: u16 = 1;
pub const AT_LISTING: usize = 56;
pub const AT_RELEASE: usize = 88;

const LINUX: &str = "linux.";
/// The listings a Qwen tier the personality ships is sold under, a namespace
/// no package of the catalogue may use, and the one program each installs:
/// the image's own chat window, never a download. A release is the vendor's
/// only with both; a package_url merely starting with "store:" is not.
pub const TIER_LISTING: &str = "linux.qwen-";
pub const TIER_PACKAGE: &str = "store:/bin/qwenchat";

pub fn policy_verdict(
    k: &impl Keccak,
    entry: &MarketplaceEntry,
    rel: &CapsuleRelease,
) -> PolicyVerdict {
    if !entry.listing_id.starts_with(LINUX) {
        return PolicyVerdict::NotLinux;
    }
    let vendor = entry.listing_id.starts_with(TIER_LISTING) && rel.package_url == TIER_PACKAGE;
    let p = rel.ext.as_ref().map(|x| x.guest_policy.as_slice()).unwrap_or(&[]);
    if p.is_empty() {
        return if vendor { PolicyVerdict::Vendor } else { PolicyVerdict::Missing };
    }
    if p.len() != POLICY_LEN || p[..8] != POLICY_MAGIC || p[8..10] != POLICY_VERSION.to_le_bytes() {
        return PolicyVerdict::BadHeader;
    }
    let (want_listing, want_release) = if vendor {
        ([0; 32], [0; 32])
    } else {
        let listing = listing_id(k, &entry.publisher_pubkey, &entry.listing_id);
        (listing, release_id(k, &listing, &rel.release_id))
    };
    let ids_ok = p[AT_LISTING..AT_LISTING + 32] == want_listing
        && p[AT_RELEASE..AT_RELEASE + 32] == want_release;
    match (ids_ok, vendor) {
        (false, _) => PolicyVerdict::WrongIds,
        (true, true) => PolicyVerdict::Vendor,
        (true, false) => PolicyVerdict::Valid,
    }
}
