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

//! The box check before any ask, a container put together from its GETs,
//! and the offer made once per listing.

use super::proof_box::{judge, AT_LISTING};
use super::proof_fail::Fail;
use super::proof_get::{Assembly, GET_MAX};
use super::proof_offer::{offerable, Offered};

fn arg(named: u8, key: u8) -> [u8; 320] {
    let mut m = [0x11u8; 320];
    m[AT_LISTING..AT_LISTING + 32].fill(named);
    m[256..288].fill(key);
    m
}

#[test]
fn the_box_must_name_this_listing_or_none() {
    let (policy, key) = judge(Some(&arg(0xaa, 0xaa))).expect("names it");
    assert_eq!((policy[AT_LISTING], policy[0], key), (0xaa, 0x11, [0xaa; 32]));
    assert!(judge(Some(&arg(0, 0xaa))).is_ok(), "names no listing");
    assert_eq!(judge(Some(&arg(0xbb, 0xaa))), Err(Fail::NotNamed));
    assert_eq!(judge(None), Err(Fail::NoBox));
}

#[test]
fn a_container_is_its_chunks_in_order_and_nothing_else() {
    assert!(Assembly::new(0).is_none() && Assembly::new((1 << 20) + 1).is_none());
    let mut a = Assembly::new(GET_MAX as u32 + 3).expect("fits");
    assert!(!a.take(&[]), "empty");
    assert!(!a.take(&alloc::vec![1; GET_MAX + 1]), "over GET_MAX");
    assert!(a.take(&alloc::vec![1; GET_MAX]) && !a.done());
    assert!(!a.take(&[2; 4]), "past the total");
    assert!(a.take(&[2; 3]) && a.done());
    assert_eq!((a.bytes.len(), a.bytes[GET_MAX], a.at()), (GET_MAX + 3, 2, GET_MAX as u32 + 3));
}

#[test]
fn each_listing_is_offered_once() {
    let mut o = Offered::default();
    assert!(o.first(b"linux.age"));
    assert!(!o.first(b"linux.age"), "once per listing per store session");
    assert!(o.first(b"linux.git"));
    assert!(offerable(b"linux.age", true));
    assert!(!offerable(b"linux.age", false) && !offerable(b"nonos.term", true));
}
