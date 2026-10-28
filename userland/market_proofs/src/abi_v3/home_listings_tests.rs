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

//! The listing keys capsule_linux gives a larger home (app/home_listings.rs),
//! recomputed from the catalogue listing each stands for by the abi's own
//! listing_id. A key edited by hand, or a listing republished under another
//! publisher, fails here, and that listing's wallet could not save.

use super::hosts::Host;
use nonos_marketplace_abi::{listing_id, AT_LISTING};

#[path = "../../../capsule_linux/src/linux/app/home_listings.rs"]
mod home_listings;

const MONERO: &str = include_str!("../../../../nonos-data/market/catalogue/linux.monero.json");

/// The string value of the first `key` in the listing.
fn field(key: &str) -> &'static str {
    let at = MONERO.find(&format!("\"{key}\": \"")).expect(key) + key.len() + 5;
    &MONERO[at..at + MONERO[at..].find('"').expect(key)]
}

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn the_monero_home_is_its_listings_own() {
    assert_eq!(field("listing_id"), "linux.monero");
    let publisher: [u8; 32] = hex(field("publisher_pubkey")).try_into().unwrap();
    let key = listing_id(&Host, &publisher, "linux.monero");
    assert_eq!(home_listings::MONERO, key, "not linux.monero's listing key");
    let policy = hex(field("guest_policy"));
    assert_eq!(policy[AT_LISTING..AT_LISTING + 32], key, "the policy binds another listing");
    // The program its policy names, at nonos_linux_policy's AT_PROGRAM (16).
    let program: [u8; 32] = policy[16..48].try_into().unwrap();
    assert_eq!(home_listings::MONERO_PROGRAM, program, "not the program linux.monero runs");
    assert_eq!(home_listings::LARGER, &[(key, program, 64 << 20)][..], "one listing, 64 MiB");
}
