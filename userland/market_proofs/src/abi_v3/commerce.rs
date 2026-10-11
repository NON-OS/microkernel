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

//! The publisher's signed commerce hash against the operator's listing.

use nonos_marketplace_abi::{commerce_hash_input, commerce_verdict, CommerceVerdict};

use super::fixtures::sold;
use super::hosts::Host;

#[test]
fn a_release_that_signed_the_listed_block_matches() {
    let mut index = sold();
    let entry = &mut index.entries[0];
    let hash = *blake3::hash(&commerce_hash_input(entry.commerce.as_ref().unwrap())).as_bytes();
    entry.releases[0].ext.as_mut().unwrap().commerce_hash = hash;
    assert_eq!(commerce_verdict(&Host, entry, &entry.releases[0]), CommerceVerdict::Matches);
    // The fixture's second release signed something else.
    assert_eq!(commerce_verdict(&Host, entry, &entry.releases[1]), CommerceVerdict::Mismatch);

    // The operator raising the price behind the publisher breaks the match.
    entry.commerce.as_mut().unwrap().unit_price += 1;
    assert_eq!(commerce_verdict(&Host, entry, &entry.releases[0]), CommerceVerdict::Mismatch);
}

#[test]
fn schema_2_has_no_commerce_to_match() {
    let mut index = sold();
    let entry = &mut index.entries[0];
    entry.commerce = None;
    assert_eq!(commerce_verdict(&Host, entry, &entry.releases[0]), CommerceVerdict::NotV3);
    let mut rel = entry.releases[0].clone();
    rel.ext = None;
    entry.commerce = sold().entries[0].commerce.clone();
    assert_eq!(commerce_verdict(&Host, entry, &rel), CommerceVerdict::NotV3);
}
