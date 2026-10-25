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

//! Only a shipped Qwen tier installs with no policy of its own: a listing
//! in the tiers' namespace whose one program is the image's chat window. A
//! package_url that merely starts with "store:" is any other package, and
//! needs its own policy with its own ids.

use nonos_marketplace_abi::{listing_id, policy_verdict, release_id, PolicyVerdict};
use nonos_marketplace_abi::{TIER_LISTING, TIER_PACKAGE};

use super::hosts::Host;
use super::policy_fixture::{linux, policy};

fn tier(policy: Vec<u8>, url: &str) -> (PolicyVerdict, [u8; 32], [u8; 32]) {
    let (mut e, rel) = linux(policy, url);
    e.listing_id = "linux.qwen-small".into();
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    (policy_verdict(&Host, &e, &rel), listing, release_id(&Host, &listing, &rel.release_id))
}

#[test]
fn a_shipped_tier_needs_no_policy_or_zero_ids() {
    assert_eq!((TIER_LISTING, TIER_PACKAGE), ("linux.qwen-", "store:/bin/qwenchat"));
    assert_eq!(tier(vec![], TIER_PACKAGE).0, PolicyVerdict::Vendor);
    assert_eq!(tier(policy([0; 32], [0; 32]), TIER_PACKAGE).0, PolicyVerdict::Vendor);
    let (_, listing, release) = tier(vec![], TIER_PACKAGE);
    assert_eq!(tier(policy(listing, release), TIER_PACKAGE).0, PolicyVerdict::WrongIds);
}

#[test]
fn a_store_url_outside_the_tiers_is_an_ordinary_package() {
    for url in ["store:/bin/qwenchat", "store:", "store:/bin/sh", "store:/bin/qwenchat2"] {
        let (e, rel) = linux(vec![], url);
        assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::Missing, "{url}");
        let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
        let release = release_id(&Host, &listing, &rel.release_id);
        let (e, rel) = linux(policy([0; 32], [0; 32]), url);
        assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::WrongIds, "{url}");
        let (e, rel) = linux(policy(listing, release), url);
        assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::Valid, "{url}");
    }
}

#[test]
fn a_tier_listing_with_another_program_is_not_the_vendors() {
    for url in ["store:/bin/sh", "store:", "https://pkg", "store:/bin/qwenchat "] {
        assert_eq!(tier(vec![], url).0, PolicyVerdict::Missing, "{url}");
    }
}
