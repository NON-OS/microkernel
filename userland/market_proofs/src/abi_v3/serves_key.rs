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

//! The listing key the kernel pairs the SimpleX window with (SERVES,
//! src/services/registry/serves.rs), recomputed from the catalogue listing it
//! stands for by the abi's own listing_id, and held to the id that listing's
//! guest policy carries, which the personality binds at start. A key edited
//! by hand, or a listing republished under another key, fails here.

use super::hosts::Host;
use nonos_marketplace_abi::{listing_id, AT_LISTING};

#[path = "../../../../src/services/registry/serves.rs"]
mod serves;

const LISTING: &str =
    include_str!("../../../../nonos-data/market/catalogue/linux.kali.nonos-simplex-chat.json");

/// The string value of the first `key` in the listing.
fn field(key: &str) -> &'static str {
    let at = LISTING.find(&format!("\"{key}\": \"")).expect(key) + key.len() + 5;
    &LISTING[at..at + LISTING[at..].find('"').expect(key)]
}

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn the_simplex_window_is_paired_with_its_listings_key() {
    let publisher: [u8; 32] = hex(field("publisher_pubkey")).try_into().unwrap();
    let key = listing_id(&Host, &publisher, field("listing_id"));
    let (_, served) = serves::SERVES.iter().find(|(c, _)| *c == "app.simplex").unwrap();
    assert_eq!(*served, key, "SERVES' key is not linux.kali.nonos-simplex-chat's");
    let policy = hex(field("guest_policy"));
    assert_eq!(policy[AT_LISTING..AT_LISTING + 32], key, "the policy binds another listing");
}
