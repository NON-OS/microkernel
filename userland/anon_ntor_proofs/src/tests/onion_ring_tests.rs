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
//! The onion address, time periods, blinding and the HSDir ring, held to the
//! fork's own known answers in src/test/test_hs_common.c.

use alloc::vec::Vec;

use crate::hex;
use crate::onion::address::{is_onion, parse};
use crate::onion::blind::{blinded_key, subcredential};
use crate::onion::period::Clock;
use crate::onion::ring::{node_index, responsible, store_index, REPLICAS, SPREAD_FETCH};

fn k32(text: &str) -> [u8; 32] {
    hex(text).try_into().expect("32 bytes")
}

/// test_build_address: the fork's checksum prefix gives "...enctcid".
const TOR_KEY: &str = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
const TOR_ADDR: &str = "25njqamcweflpvkl73j4szahhihoc4xt3ktcgjnpaingr5yhkenctcid";

#[test]
fn the_forks_address_vector() {
    let host = alloc::format!("{TOR_ADDR}.anyone");
    assert_eq!(parse(host.as_bytes()), Some(k32(TOR_KEY)));
    let upper = host.to_ascii_uppercase();
    assert_eq!(parse(upper.as_bytes()), Some(k32(TOR_KEY)));
    let sub = alloc::format!("www.{TOR_ADDR}.anyone");
    assert_eq!(parse(sub.as_bytes()), Some(k32(TOR_KEY)));
}

#[test]
fn addresses_that_are_refused() {
    /* Tor's own checksum for the same key: valid as .onion, not here. */
    assert_eq!(parse(b"25njqamcweflpvkl73j4szahhihoc4xt3ktcgjnpaingr5yhkenl5sid.anyone"), None);
    assert_eq!(parse(alloc::format!("{TOR_ADDR}.onion").as_bytes()), None);
    assert_eq!(parse(&alloc::format!("{}.anyone", &TOR_ADDR[1..]).into_bytes()), None);
    assert_eq!(parse(b"anyone.anyone"), None);
    assert_eq!(parse(b".anyone"), None);
    let mut bad = TOR_ADDR.as_bytes().to_vec();
    bad[0] = b'1';
    bad.extend_from_slice(b".anyone");
    assert_eq!(parse(&bad), None);
    /* Version byte changed: the last character carries it. */
    let mut v = TOR_ADDR.as_bytes().to_vec();
    *v.last_mut().unwrap() = b'a';
    v.extend_from_slice(b".anyone");
    assert_eq!(parse(&v), None);
    assert!(is_onion(b"foo.anyone") && is_onion(b"X.ANYONE"));
    /* The root's trailing dot: still an onion name, never an exit's host. */
    assert!(is_onion(b"foo.anyone.") && is_onion(b"anything.ANYONE."));
    assert_eq!(parse(alloc::format!("{TOR_ADDR}.anyone.").as_bytes()), Some(k32(TOR_KEY)));
    /* Two dots: caught as an onion name, so never sent out, and refused. */
    assert!(is_onion(alloc::format!("{TOR_ADDR}.anyone..").as_bytes()));
    assert_eq!(parse(alloc::format!("{TOR_ADDR}.anyone..").as_bytes()), None);
    assert!(!is_onion(b"example.com.") && !is_onion(b"anyone."));
    assert!(!is_onion(b"example.com") && !is_onion(b"anyone"));
}

/// test_time_period and test_time_between_tp_and_srv, with the one hour
/// voting interval those tests use.
#[test]
fn the_forks_time_period_vectors() {
    let at = |t: u64| Clock { valid_after: t, interval: 3600 };
    assert_eq!(at(1460545200).period(), Some(16903));
    assert_eq!(at(1460548799).period(), Some(16903));
    assert_eq!(at(1460548800).period(), Some(16904));
    /* hs_in_period_between_tp_and_srv: 0, 0, 1, 1, 0. current_srv is it. */
    for (t, current) in [
        (499132800u64, false),
        (499172400, false),
        (499176000, true),
        (499215600, true),
        (499219200, false),
    ] {
        assert_eq!(at(t).current_srv(), Some(current), "{t}");
    }
    assert_eq!(Clock { valid_after: 10, interval: 0 }.current_srv(), None);
}

/// test_blinding_basics: time period 1234 is 1973-05-20 01:50:33.
#[test]
fn the_forks_blinding_vector() {
    assert_eq!(Clock { valid_after: 106710633, interval: 3600 }.period(), Some(1234));
    let identity = k32("833990B085C1A688C1D4C8B1F6B56AFAF5A2ECA674449E1D704F83765CCB7BC6");
    let blinded = blinded_key(&identity, 1234, 1440).expect("on the curve");
    assert_eq!(blinded.to_vec(), hex("3A50BF210E8F9EE955AE0014F7A6917FB65EBF098A86305ABB508D1A7291B6D5"));
    assert_eq!(
        subcredential(&identity, &blinded).to_vec(),
        hex("635D55907816E8D76398A675A50B1C2F3E36B42A5CA77BA3A0441285161AE07D")
    );
}

/// test_hs_indexes.
#[test]
fn the_forks_ring_index_vectors() {
    let key = [0x42u8; 32];
    assert_eq!(
        store_index(&key, 1, 42, 1440).to_vec(),
        hex("37e5cbbd56a22823714f18f1623ece5983a0d64c78495a8cfab854245e5f9a8a")
    );
    assert_eq!(
        node_index(&key, &[0x43u8; 32], 42, 1440).to_vec(),
        hex("db475361014a09965e7e5e4d4a25b8f8d4b8f16cb1d8a7e95eed50249cc1a2d5")
    );
}

/*
 * The walk round the ring: each replica takes the next SPREAD_FETCH distinct
 * nodes at or after its index, wrapping, and a node two replicas land on
 * counts once. Checked against a brute force over a fixed ring.
 */
#[test]
fn responsible_walks_the_ring_from_each_store_index() {
    let blinded = [7u8; 32];
    let nodes: Vec<[u8; 32]> =
        (0u8..20).map(|i| crate::keccak::sha3_256_parts(&[&[i]])).collect();
    let picked = responsible(&nodes, &blinded, 99, 1440);

    let mut order: Vec<usize> = (0..nodes.len()).collect();
    order.sort_by(|a, b| nodes[*a].cmp(&nodes[*b]));
    let mut want: Vec<usize> = Vec::new();
    for replica in 1..=REPLICAS {
        let target = store_index(&blinded, replica, 99, 1440);
        let start = order.iter().position(|i| nodes[*i] >= target).unwrap_or(0);
        let mut taken = 0;
        let mut step = 0;
        while taken < SPREAD_FETCH && step < order.len() {
            let node = order[(start + step) % order.len()];
            if !want.contains(&node) {
                want.push(node);
                taken += 1;
            }
            step += 1;
        }
    }
    assert_eq!(picked, want);
    assert_eq!(picked.len(), 6);

    assert!(responsible(&[], &blinded, 99, 1440).is_empty());
    assert_eq!(responsible(&nodes[..2], &blinded, 99, 1440).len(), 2, "a ring smaller than the spread");
}

/*
 * The lookup picks its shared random value by the clock: at 19:00 a fetch
 * uses the current value, at 11:00 the previous one, and with neither in the
 * consensus the disaster value for the period.
 */
#[test]
fn a_lookup_uses_the_value_the_clock_names() {
    use crate::onion::lookup::Lookup;
    use crate::onion::ring::disaster_srv;
    let identity = k32(TOR_KEY);
    let ids: Vec<[u8; 32]> = (0u8..30).map(|i| crate::keccak::sha3_256_parts(&[&[i, 1]])).collect();
    let current = [1u8; 32];
    let previous = [2u8; 32];
    let ring = |srv: &[u8; 32], period| -> Vec<[u8; 32]> {
        ids.iter().map(|id| node_index(id, srv, period, 1440)).collect()
    };

    let evening = Clock { valid_after: 1789758000, interval: 3600 };
    let l = Lookup::new(&identity, &evening).expect("lookup");
    assert_eq!(l.period, 20714);
    assert_eq!(l.blinded, blinded_key(&identity, 20714, 1440).unwrap());
    assert_eq!(
        l.hsdirs(&evening, Some(current), Some(previous), &ids),
        Some(responsible(&ring(&current, 20714), &l.blinded, 20714, 1440))
    );

    let morning = Clock { valid_after: 1789729200, interval: 3600 };
    let m = Lookup::new(&identity, &morning).expect("lookup");
    assert_eq!(m.period, 20713);
    assert_eq!(
        m.hsdirs(&morning, Some(current), Some(previous), &ids),
        Some(responsible(&ring(&previous, 20713), &m.blinded, 20713, 1440))
    );
    assert_eq!(
        m.hsdirs(&morning, Some(current), None, &ids),
        Some(responsible(&ring(&disaster_srv(20713, 1440), 20713), &m.blinded, 20713, 1440))
    );
}

/* The shield's landers on NONOS (userland/shield_core/src/net/pools.rs), each a
 * whole .anyone address with a good checksum, so a letter mistyped in the list
 * fails here and not on a payment. */
#[test]
fn the_shield_landers_are_good_anyone_addresses() {
    let pools = include_str!("../../../shield_core/src/net/pools.rs");
    let landers: Vec<&str> = pools
        .split('"')
        .filter(|s| s.ends_with(".anyone"))
        .collect();
    assert_eq!(landers.len(), 4, "the four standby landers");
    for host in landers {
        assert!(is_onion(host.as_bytes()), "{host}");
        assert!(parse(host.as_bytes()).is_some(), "{host}: checksum or version");
    }
}
