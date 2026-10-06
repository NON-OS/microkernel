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
//! Join.
//! The descriptor cache's rules, on fixed entries.

use alloc::vec::Vec;

use crate::onion::cache::{CacheKey, DescCache, CACHE_MAX};

fn key(blinded: [u8; 32], period: u64) -> CacheKey {
    CacheKey { identity: [1; 32], blinded, period }
}
use crate::onion::desc::IntroPoint;

fn points(tag: u8) -> Vec<IntroPoint> {
    alloc::vec![IntroPoint {
        address: [tag, 0, 0, 1],
        port: 9001,
        rsa_identity: [tag; 20],
        ed25519_identity: [tag; 32],
        onion_key: [tag; 32],
        auth_key: [tag; 32],
        enc_key: [tag; 32],
    }]
}

const B: [u8; 32] = [7; 32];

#[test]
fn an_entry_lives_for_the_descriptors_lifetime() {
    let mut c = DescCache::default();
    assert!(c.insert(key(B, 10), 1, 3600, 1000, points(1), None));
    assert_eq!(c.get(&B, 10, 1000).unwrap().0[0].address[0], 1);
    assert!(c.get(&B, 10, 4599).is_some());
    assert!(c.get(&B, 10, 4600).is_none(), "gone at its lifetime");
}

#[test]
fn an_entry_belongs_to_one_period() {
    let mut c = DescCache::default();
    c.insert(key(B, 10), 1, 3600, 0, points(1), None);
    assert!(c.get(&B, 11, 0).is_none(), "never served for another period");
    c.prune(11, 0);
    assert!(c.get(&B, 10, 0).is_none(), "a period change wipes it");
}

#[test]
fn a_newer_revision_replaces_an_older_one_and_never_the_reverse() {
    let mut c = DescCache::default();
    assert!(c.insert(key(B, 10), 5, 3600, 0, points(1), None));
    assert!(!c.insert(key(B, 10), 4, 3600, 0, points(2), None), "an older revision is refused");
    assert_eq!(c.get(&B, 10, 0).unwrap().0[0].address[0], 1);
    assert!(c.insert(key(B, 10), 5, 3600, 0, points(3), None), "the same revision refreshes it");
    assert!(c.insert(key(B, 10), 6, 3600, 0, points(4), None));
    assert_eq!(c.get(&B, 10, 0).unwrap().0[0].address[0], 4);
}

#[test]
fn the_cache_is_bounded_and_evicts_the_soonest_to_expire() {
    let mut c = DescCache::default();
    for i in 0..CACHE_MAX as u8 {
        c.insert(key([i; 32], 10), 1, 1000 + i as u64, 0, points(i), None);
    }
    c.insert(key([200; 32], 10), 1, 5000, 0, points(200), None);
    assert!(c.get(&[0; 32], 10, 0).is_none(), "the soonest to expire went");
    for i in 1..CACHE_MAX as u8 {
        assert!(c.get(&[i; 32], 10, 0).is_some());
    }
    assert!(c.get(&[200; 32], 10, 0).is_some());
}

#[test]
fn forget_drops_one_service() {
    let mut c = DescCache::default();
    c.insert(key(B, 10), 1, 3600, 0, points(1), None);
    c.insert(key([8; 32], 10), 1, 3600, 0, points(2), None);
    c.forget(&B, 10);
    assert!(c.get(&B, 10, 0).is_none());
    assert!(c.get(&[8; 32], 10, 0).is_some());
}
