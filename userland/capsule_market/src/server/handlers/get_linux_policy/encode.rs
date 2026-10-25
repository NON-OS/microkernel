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

//! What `OP_GET_LINUX_POLICY` answers for one release (I-C5, plan/30): the
//! policy and the two keys the install role checks it against, only when
//! the policy gate passes as Valid and the release is install ready, which
//! holds both signatures; nothing for a vendor program or a listing that is
//! not Linux; a refusal otherwise.

extern crate alloc;

use alloc::boxed::Box;

use nonos_market_proto::{LinuxPolicy, MARKET_ARG_LEN};
use nonos_marketplace_abi::{listing_id, policy_verdict, release_id, PolicyVerdict};
use nonos_marketplace_abi::{CapsuleRelease, MarketplaceEntry, POLICY_LEN};

use crate::chain_hosts::Host;
use crate::install_ready::ready;

/// Whether the index and this release's publisher signature verified: a
/// policy nobody signed is not the publisher's box, and one for a release
/// readiness refuses is no box anything installs in.
pub(super) fn encode(
    entry: &MarketplaceEntry,
    rel: &CapsuleRelease,
    index_signed: bool,
    publisher_signed: bool,
) -> LinuxPolicy {
    match policy_verdict(&Host, entry, rel) {
        PolicyVerdict::NotLinux | PolicyVerdict::Vendor => LinuxPolicy::NoPolicy,
        PolicyVerdict::Valid if ready(index_signed, entry, rel, publisher_signed) => {
            arg(entry, rel).map_or(LinuxPolicy::Refused, LinuxPolicy::Policy)
        }
        _ => LinuxPolicy::Refused,
    }
}

fn arg(entry: &MarketplaceEntry, rel: &CapsuleRelease) -> Option<Box<[u8; MARKET_ARG_LEN]>> {
    let policy: &[u8; POLICY_LEN] = rel.ext.as_ref()?.guest_policy.as_slice().try_into().ok()?;
    let listing = listing_id(&Host, &entry.publisher_pubkey, &entry.listing_id);
    let release = release_id(&Host, &listing, &rel.release_id);
    let mut m = Box::new([0u8; MARKET_ARG_LEN]);
    let (p, keys) = m.split_at_mut(POLICY_LEN);
    let (l, r) = keys.split_at_mut(32);
    p.copy_from_slice(policy);
    l.copy_from_slice(&listing);
    r.copy_from_slice(&release);
    Some(m)
}
