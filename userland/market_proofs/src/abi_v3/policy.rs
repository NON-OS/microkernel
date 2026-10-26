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

//! The guest policy gate, against the plan's layout written out here.

use nonos_marketplace_abi::{listing_id, policy_verdict, release_id, PolicyVerdict};
use nonos_marketplace_abi::{AT_LISTING, AT_RELEASE, POLICY_LEN, POLICY_MAGIC, POLICY_VERSION};

use super::hosts::Host;
use super::policy_fixture::{linux, policy};

#[test]
fn the_pinned_offsets_are_plan_30s() {
    assert_eq!((POLICY_LEN, &POLICY_MAGIC, POLICY_VERSION), (256, b"NLXPOL01", 1));
    assert_eq!((AT_LISTING, AT_RELEASE), (56, 88));
}

#[test]
fn a_market_package_needs_its_own_ids() {
    let (e, rel) = linux(vec![], "https://pkg");
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    let release = release_id(&Host, &listing, &rel.release_id);
    assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::Missing);
    let (e, rel) = linux(policy(listing, release), "https://pkg");
    assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::Valid);
    let (e, rel) = linux(policy(listing, [0; 32]), "https://pkg");
    assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::WrongIds);
}

#[test]
fn a_wrong_header_is_refused_and_a_non_linux_listing_is_not_asked() {
    for at in [0, 7, 8, 9] {
        let mut p = policy([1; 32], [2; 32]);
        p[at] ^= 1;
        let (e, rel) = linux(p, "https://pkg");
        assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::BadHeader, "byte {at}");
    }
    let (e, rel) = linux(vec![0; 255], "https://pkg");
    assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::BadHeader);
    let (mut e, rel) = linux(vec![], "https://pkg");
    e.listing_id = "nonos.app".into();
    assert_eq!(policy_verdict(&Host, &e, &rel), PolicyVerdict::NotLinux);
}
