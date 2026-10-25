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

//! OP_GET_LINUX_POLICY's answer for a release: its box only when the release
//! is install ready, for the release found, with that release's hash and
//! signatures.

use nonos_market_proto::LinuxPolicy;
use nonos_marketplace_abi::{listing_id, release_id};

use super::hosts::Host;
use super::linux_policy_answer::answer;
use super::policy_fixture::{linux, policy};

/* The answer is for the release the market found, by the same resolution as
 * readiness, with that release's package hash; each signature is the
 * accepted index's own. */
#[test]
fn the_answer_is_for_the_release_found_with_its_hash_and_signatures() {
    let (e, rel) = linux(vec![], "https://pkg");
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    let release = release_id(&Host, &listing, &rel.release_id);
    let (e, rel) = linux(policy(listing, release), "https://pkg");
    let mut index = super::fixtures::sold();
    index.entries =
        vec![nonos_marketplace_abi::MarketplaceEntry { releases: vec![rel.clone()], ..e.clone() }];
    let ask = |signed, publisher: bool| answer(&index, signed, |_, _| publisher, &e.listing_id, "");
    let (reply, hash) = ask(true, true).expect("found");
    assert!(matches!(reply, LinuxPolicy::Policy(_)) && hash == rel.package_hash);
    assert_eq!(ask(false, true).map(|a| a.0), Some(LinuxPolicy::Refused), "index unsigned");
    assert_eq!(ask(true, false).map(|a| a.0), Some(LinuxPolicy::Refused), "publisher unsigned");
    assert!(answer(&index, true, |_, _| true, "linux.other", "").is_none(), "no such listing");
    assert!(
        answer(&index, true, |_, _| true, &e.listing_id, "nope@1").is_none(),
        "no such release"
    );
    let mut second = rel.clone();
    second.release_id = "pkg@2".into();
    second.package_hash = [0xab; 32];
    index.entries[0].releases.push(second);
    let (_, hash) = answer(&index, true, |_, _| true, &e.listing_id, "pkg@2").expect("the second");
    assert_eq!(hash, [0xab; 32], "the hash of the release asked for, not the first");
    let mut seen = None;
    let _ = answer(
        &index,
        true,
        |at, r| {
            seen = Some((at, r));
            true
        },
        &e.listing_id,
        "pkg@2",
    );
    assert_eq!(seen, Some((0, 1)), "the publisher flag asked of the release found");
}
