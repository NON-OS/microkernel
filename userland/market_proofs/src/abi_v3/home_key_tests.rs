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

//! The home a market program's /home and /root are kept in is its
//! listing's: two publishers who name one listing get two homes, and every
//! release of one listing keeps the one (capsule_linux app/home_key.rs).

use nonos_marketplace_abi::{listing_id, release_id};

use super::hosts::Host;

#[path = "../../../capsule_linux/src/linux/app/home_key.rs"]
mod home_key;
use home_key::home_key;

#[test]
fn two_publishers_naming_one_listing_get_disjoint_homes() {
    let (alice, mallory) = ([0xa1; 32], [0x3e; 32]);
    let (a, m) = (listing_id(&Host, &alice, "linux.age"), listing_id(&Host, &mallory, "linux.age"));
    assert_ne!(a, m, "the listing key binds the publisher");
    assert_ne!(home_key(&a, &[0; 32]), home_key(&m, &[0; 32]));
}

#[test]
fn every_release_of_a_listing_keeps_its_home_whatever_its_program() {
    let listing = listing_id(&Host, &[0xa1; 32], "linux.age");
    let (r1, r2) = (release_id(&Host, &listing, "1.1.1"), release_id(&Host, &listing, "1.2.0"));
    assert_ne!(r1, r2);
    assert_eq!(home_key(&listing, &[1; 32]), home_key(&listing, &[2; 32]));
    assert_ne!(
        home_key(&listing, &[1; 32]),
        home_key(&listing_id(&Host, &[0xa1; 32], "linux.git"), &[1; 32])
    );
}
