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

//! What the market answers to OP_GET_LINUX_POLICY (I-C5) is what init and
//! the install role read: the capsule's own encoder, through market_proto's
//! wire, against the abi's ids and the plan's layout.

use nonos_market_proto::{linux_policy_body, parse_linux_policy, LinuxPolicy};
use nonos_marketplace_abi::{listing_id, release_id};

use super::hosts::Host;
use super::linux_policy_encode::encode;
use super::policy_fixture::{linux, policy};

pub(super) fn read(reply: LinuxPolicy) -> LinuxPolicy {
    parse_linux_policy(&linux_policy_body(&reply, &[0; 32]))
        .expect("the market's own reply reads")
        .0
}

#[test]
fn a_valid_signed_policy_goes_out_whole_with_its_two_keys() {
    let (e, rel) = linux(vec![], "https://pkg");
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    let release = release_id(&Host, &listing, &rel.release_id);
    let p = policy(listing, release);
    let (e, rel) = linux(p.clone(), "https://pkg");
    let LinuxPolicy::Policy(m) = read(encode(&e, &rel, true, true)) else {
        panic!("no policy sent")
    };
    assert_eq!((&m[..256], &m[256..288], &m[288..]), (&p[..], &listing[..], &release[..]));
}

#[test]
fn a_policy_nobody_signed_or_that_fails_the_gate_is_refused() {
    let (e, rel) = linux(vec![], "https://pkg");
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    let release = release_id(&Host, &listing, &rel.release_id);
    let (e, rel) = linux(policy(listing, release), "https://pkg");
    assert_eq!(read(encode(&e, &rel, false, true)), LinuxPolicy::Refused, "index unsigned");
    assert_eq!(read(encode(&e, &rel, true, false)), LinuxPolicy::Refused, "publisher unsigned");
    for p in [vec![], policy(listing, [0; 32]), policy([0; 32], [0; 32]), vec![0; 255]] {
        let (e, rel) = linux(p, "https://pkg");
        assert_eq!(read(encode(&e, &rel, true, true)), LinuxPolicy::Refused);
    }
    let (e, rel) = linux(policy(listing, release), "store:/bin/qwenchat");
    assert_eq!(
        read(encode(&e, &rel, true, true)),
        LinuxPolicy::Refused,
        "a vendor program naming ids"
    );
}
