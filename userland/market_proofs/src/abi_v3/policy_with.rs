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

//! A release's extra packages ride after its listed arguments, or after its
//! policy, in the same tail and under the same signatures: the index and
//! the market's answer carry the tail byte for byte, and a release with
//! none carries the bytes it always has.

use nonos_market_proto::LinuxPolicy;
use nonos_marketplace_abi::{listing_id, policy_verdict, release_id, PolicyVerdict};

use super::blobs::{blob, decode};
use super::fixtures::sold;
use super::hosts::Host;
use super::linux_policy_encode::encode;
use super::linux_policy_wire::read;
use super::policy_fixture::{linux, policy};

/// As the catalogue writes them (listed_args.py, listed_with.py).
const ARGS: &[u8] = b"NLXARG01\x01\x05--off";
const WITH: &[u8] = b"NLXWITH1\x02\x11zathura-pdf-mupdf\x0cpoppler-data";

fn sold_with(tail: &[u8]) -> Vec<u8> {
    let (e, rel) = linux(vec![], "https://pkg");
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    [&policy(listing, release_id(&Host, &listing, &rel.release_id))[..], tail].concat()
}

#[test]
fn the_extras_go_out_after_the_arguments_or_alone_byte_for_byte() {
    let both = [ARGS, WITH].concat();
    for tail in [&both[..], WITH, ARGS, b""] {
        let (e, rel) = linux(sold_with(tail), "https://pkg");
        assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::Valid, "{tail:?}");
        let LinuxPolicy::Policy(m, sent) = read(encode(&e, &rel, true, true)) else {
            panic!("{tail:?}")
        };
        assert_eq!((&m[..256], &sent[..]), (&sold_with(b"")[..], tail));
    }
}

#[test]
fn the_index_carries_the_tail_whole() {
    let mut index = sold();
    let ext = index.entries[0].releases[0].ext.as_mut().unwrap();
    ext.guest_policy = sold_with(&[ARGS, WITH].concat());
    let back = decode(&blob(index.clone())).expect("reads");
    assert_eq!(back.entries[0].releases[0].ext, index.entries[0].releases[0].ext);
}
