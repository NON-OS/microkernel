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

//! Which releases OP_GET_LINUX_POLICY sends no box for, and which it refuses
//! one: a vendor program or a listing that is not Linux has none of its own,
//! and a release init would not install gets none.

use nonos_market_proto::LinuxPolicy;
use nonos_marketplace_abi::{listing_id, release_id};

use super::hosts::Host;
use super::linux_policy_encode::encode;
use super::linux_policy_wire::read;
use super::policy_fixture::{linux, policy};

/* The box goes out only for a release init would install: a valid, signed
 * policy for a release readiness refuses is still refused. */
#[test]
fn a_policy_for_a_release_that_is_not_ready_is_refused() {
    let (e, rel) = linux(vec![], "https://pkg");
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    let release = release_id(&Host, &listing, &rel.release_id);
    let (e, mut rel) = linux(policy(listing, release), "https://pkg");
    assert!(matches!(read(encode(&e, &rel, true, true)), LinuxPolicy::Policy(_)));
    rel.supported_arches = vec!["riscv64".into()];
    assert_eq!(read(encode(&e, &rel, true, true)), LinuxPolicy::Refused, "another arch");
    let (mut sale, rel) = linux(policy(listing, release), "https://pkg");
    sale.commerce = super::fixtures::sold().entries[0].commerce.clone();
    assert_eq!(read(encode(&sale, &rel, true, true)), LinuxPolicy::Refused, "a sale with no bond");
}

#[test]
fn a_vendor_program_or_a_listing_not_linux_sends_no_policy() {
    for p in [vec![], policy([0; 32], [0; 32])] {
        let (mut e, rel) = linux(p.clone(), "store:/bin/qwenchat");
        e.listing_id = "linux.qwen-small".into();
        assert_eq!(read(encode(&e, &rel, true, true)), LinuxPolicy::NoPolicy);
        /* The same program under any other listing is not the vendor's. */
        let (e, rel) = linux(p, "store:/bin/qwenchat");
        assert_eq!(read(encode(&e, &rel, true, true)), LinuxPolicy::Refused);
    }
    let (mut e, rel) = linux(vec![], "https://pkg");
    e.listing_id = "nonos.app.calculator".into();
    assert_eq!(read(encode(&e, &rel, true, true)), LinuxPolicy::NoPolicy);
}
