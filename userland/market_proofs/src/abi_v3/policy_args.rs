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

//! A market release's listed arguments ride after its policy in
//! guest_policy, under the same signatures: up to GUEST_ARGS_MAX bytes, never
//! after a vendor tier's policy, and handed on whole in the market's answer.

use nonos_market_proto::LinuxPolicy;
use nonos_marketplace_abi::{encode_and_sign, EncodeError, GUEST_ARGS_MAX, TIER_PACKAGE};
use nonos_marketplace_abi::{listing_id, policy_verdict, release_id, PolicyVerdict};

use super::blobs::{blob, decode};
use super::fixtures::sold;
use super::hosts::Host;
use super::linux_policy_encode::encode;
use super::linux_policy_wire::read;
use super::policy_fixture::{linux, policy};

const MOST: usize = GUEST_ARGS_MAX as usize;

fn with_args(n: usize) -> Vec<u8> {
    let (e, rel) = linux(vec![], "https://pkg");
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    [policy(listing, release_id(&Host, &listing, &rel.release_id)), vec![5; n]].concat()
}

#[test]
fn arguments_after_a_market_policy_go_out_with_it_up_to_their_bound() {
    for n in [1, MOST] {
        let (e, rel) = linux(with_args(n), "https://pkg");
        assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::Valid);
        let LinuxPolicy::Policy(m, args) = read(encode(&e, &rel, true, true)) else {
            panic!("{n}")
        };
        assert_eq!((&m[..256], args), (&with_args(0)[..], vec![5; n]));
    }
    let (e, rel) = linux(with_args(MOST + 1), "https://pkg");
    assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::BadHeader);
    let (mut e, rel) = linux([policy([0; 32], [0; 32]), vec![5; 4]].concat(), TIER_PACKAGE);
    e.listing_id = "linux.qwen-small".into();
    assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::BadHeader, "a tier lists none");
}

#[test]
fn the_index_carries_them_and_refuses_past_the_bound() {
    let mut index = sold();
    index.entries[0].releases[0].ext.as_mut().unwrap().guest_policy =
        [vec![1; 256], vec![2; MOST]].concat();
    let back = decode(&blob(index.clone())).expect("reads");
    assert_eq!(back.entries[0].releases[0].ext, index.entries[0].releases[0].ext);
    for len in [MOST + 257, 255, 1] {
        let mut past = sold();
        past.entries[0].releases[0].ext.as_mut().unwrap().guest_policy = vec![1; len];
        assert_eq!(encode_and_sign(past, |_| [0; 64]).err(), Some(EncodeError::BadLength), "{len}");
    }
}
