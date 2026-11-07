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
//! publisher, fails here, and that listing's home would be the ordinary one.

use super::hosts::Host;
use nonos_marketplace_abi::{listing_id, AT_LISTING};

#[path = "../../../capsule_linux/src/linux/app/home_listings.rs"]
mod home_listings;

const MONERO: &str = include_str!("../../../../nonos-data/market/catalogue/linux.monero.json");
const TELEGRAM: &str =
    include_str!("../../../../nonos-data/market/catalogue/linux.telegram-desktop.json");
const TELEGRAM_IMAGE: &str =
    include_str!("../../../../nonos-data/market/images/linux.nonos-image-telegram-desktop.json");

/// The string value of the first `key` in `listing`.
fn field(listing: &'static str, key: &str) -> &'static str {
    let at = listing.find(&format!("\"{key}\": \"")).expect(key) + key.len() + 5;
    &listing[at..at + listing[at..].find('"').expect(key)]
}

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

/// The listing key and the program of the catalogue listing `name`, from its
/// publisher and its policy, each checked against the other.
fn listed(listing: &'static str, name: &str) -> ([u8; 32], [u8; 32]) {
    assert_eq!(field(listing, "listing_id"), name);
    let publisher: [u8; 32] = hex(field(listing, "publisher_pubkey")).try_into().unwrap();
    let key = listing_id(&Host, &publisher, name);
    let policy = hex(field(listing, "guest_policy"));
    assert_eq!(policy[AT_LISTING..AT_LISTING + 32], key, "{name}: the policy binds another listing");
    // The program its policy names, at nonos_linux_policy's AT_PROGRAM (16).
    (key, policy[16..48].try_into().unwrap())
}

#[test]
fn the_monero_home_is_its_listings_own() {
    let (key, program) = listed(MONERO, "linux.monero");
    assert_eq!(home_listings::MONERO, key, "not linux.monero's listing key");
    assert_eq!(home_listings::MONERO_PROGRAM, program, "not the program linux.monero runs");
    assert_eq!(home_listings::MONERO_HOME, 64 << 20);
}

#[test]
fn the_telegram_home_is_its_listings_own_and_its_images() {
    let (key, program) = listed(TELEGRAM, "linux.telegram-desktop");
    assert_eq!(home_listings::TELEGRAM, key, "not linux.telegram-desktop's listing key");
    assert_eq!(home_listings::TELEGRAM_PROGRAM, program, "not the program it runs");
    // The app image of the same release runs the same program, so its home
    // is the larger one too.
    assert_eq!(hex(field(TELEGRAM_IMAGE, "program_blake3")), program, "the image runs another");
    assert_eq!(home_listings::TELEGRAM_HOME, 64 << 20);
}

#[test]
fn only_these_two_keep_more() {
    let monero = listed(MONERO, "linux.monero");
    let telegram = listed(TELEGRAM, "linux.telegram-desktop");
    let want = [(monero.0, monero.1, 64 << 20), (telegram.0, telegram.1, 64 << 20)];
    assert_eq!(home_listings::LARGER, &want[..], "two listings, 64 MiB each");
}
