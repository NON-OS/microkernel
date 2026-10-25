// NONOS Operating System (AGPL-3.0-or-later)
//! The receive walk over seeded NTBs, built sound and then told one lie: it
//! takes exactly what the rule in ntb_sweep/model.rs takes and exactly the
//! honest datagrams the lie leaves, each wholly in the block, at least an
//! Ethernet header long, none twice and forty at most.

use std::collections::VecDeque;

use crate::ncm::ntb_in::datagrams;
use crate::ntb_sweep::build::block;
use crate::ntb_sweep::lie::{keep, tell, LIES};
use crate::ntb_sweep::model;
use crate::ntb_sweep::rng::{rounds, Rng};

fn driver(raw: &[u8], rx_max: usize, offered32: bool) -> Vec<(usize, usize)> {
    let mut q = VecDeque::new();
    datagrams(raw, rx_max, offered32, &mut q);
    q.into_iter().collect()
}

#[test]
fn every_sound_block_gives_every_datagram_in_chain_order() {
    let mut sizes = [0u32; 13];
    for seed in 1..rounds(20_000) {
        let b = block(&mut Rng::new(seed), false);
        let all = keep(&b, &|_, _| true);
        assert_eq!(driver(&b.raw, b.rx_max, true), all, "seed {seed}");
        assert_eq!(model::datagrams(&b.raw, b.rx_max, true), all, "seed {seed}: the rule");
        sizes[b.raw.len().next_power_of_two().ilog2() as usize] += 1;
    }
    for (k, &n) in sizes.iter().enumerate().skip(7) {
        assert!(cfg!(miri) || n >= 500, "only {n} blocks near {} bytes", 1 << k);
    }
}

#[test]
fn after_any_lie_the_walk_takes_exactly_the_honest_datagrams_left() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(60_000) {
        let t = tell(&mut Rng::new(seed));
        let got = driver(&t.b.raw, t.b.rx_max, t.offered32);
        let why = format!("seed {seed}: {:?}", t.lie);
        assert_eq!(got, model::datagrams(&t.b.raw, t.b.rx_max, t.offered32), "{why}");
        assert_eq!(got, t.expect, "{why}: the honest datagrams left");
        assert!(got.len() <= 40, "{why}");
        for (k, &(i, n)) in got.iter().enumerate() {
            assert!(n >= 14 && i + n <= t.b.raw.len(), "{why}: {i}+{n}");
            assert!(!got[..k].contains(&(i, n)), "{why}: taken twice");
        }
        told[LIES.iter().position(|l| *l == t.lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 1_000, "{lie:?} was told only {n} times");
    }
}
