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

//! Row 1.12: the PMKSA cache names a passphrase by an HMAC keyed by 32
//! octets the station drew from its randomness when it started, not by
//! anything public. A station with no randomness keeps no PMKSA. The tag
//! is never logged (it has no `Debug`) and never leaves the station.

use super::leak::leaked;
use crate::pmksa::cache_tests::{sa, STA};
use crate::soft::join::{joined, PASS, SSID};
use crate::soft::rig::SimAir;
use nonos_wifi_core::pmksa::PmksaCache;
use nonos_wifi_core::softmac::{Env, SoftStation};
use nonos_wifi_core::vault::Vault;
use nonos_wifi_core::wpa::sha256::hmac_sha256_parts;

struct NoRandom;

impl Env for NoRandom {
    fn now_ms(&self) -> u64 {
        0
    }
    fn random(&mut self, _out: &mut [u8]) -> bool {
        false
    }
}

#[test]
fn the_tag_depends_on_the_drawn_key_not_only_the_name_and_passphrase() {
    let (mut a, b) = (PmksaCache::new(Vault::new(&[1; 32])), PmksaCache::new(Vault::new(&[2; 32])));
    let (ta, tb) = (a.tag(SSID, PASS).unwrap(), b.tag(SSID, PASS).unwrap());
    a.keep(STA, SSID, &tb, sa(1), 0);
    assert!(a.find((sa(1).bssid, STA), SSID, &ta, 1).is_none(), "another key, another tag");
    assert!(a.find((sa(1).bssid, STA), SSID, &tb, 1) == Some(sa(1)));
}

#[test]
fn without_randomness_no_pmksa_is_kept() {
    let mut st: SoftStation<SimAir, NoRandom> = SoftStation::new(None, NoRandom, 0);
    assert!(st.pmksa().tag(SSID, PASS).is_none());
    assert!(PmksaCache::default().tag(SSID, PASS).is_none(), "no key, no tag");
    let mut c = PmksaCache::default();
    let t = PmksaCache::new(Vault::new(&[1; 32])).tag(SSID, PASS).unwrap();
    c.keep(STA, SSID, &t, sa(1), 0);
    assert!(c.is_empty(), "and a keyless cache keeps nothing");
}

#[test]
fn the_tag_never_leaves_the_station() {
    let mut st = joined();
    // The simulation's randomness is a fixed sequence; its first 32 draws
    // are the tag key the station took when it started.
    let key: Vec<u8> = (1..=32u8).map(|i| i.wrapping_mul(151).wrapping_add(7)).collect();
    let tag = hmac_sha256_parts(&key, &[&[SSID.len() as u8], SSID, PASS]);
    let kept = st.session().unwrap().kit.as_ref().unwrap().tag.is_some();
    assert!(kept, "the session holds a tag");
    assert!(!leaked(&mut st, &tag));
    assert!(!leaked(&mut st, &key), "nor its key");
}
