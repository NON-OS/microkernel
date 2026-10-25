// NONOS Operating System (AGPL-3.0-or-later)
//! The walk over configurations built sound and then told one lie: it
//! gives each descriptor it owes as built, reads only the buffer's own
//! bytes, and ends.

use crate::desc::walk;
use crate::sweep::build::config;
use crate::sweep::lie::{tell, LIES};
use crate::sweep::rng::{rounds, Rng};

#[test]
fn every_sound_configuration_walks_as_built() {
    let mut sizes = [0u32; 13];
    for seed in 1..rounds(20_000) {
        let built = config(&mut Rng::new(seed));
        let raw = built.concat();
        let got: Vec<&[u8]> = walk(&raw).collect();
        assert_eq!(got, built.iter().map(Vec::as_slice).collect::<Vec<_>>(), "seed {seed}");
        sizes[raw.len().next_power_of_two().trailing_zeros() as usize] += 1;
    }
    if !cfg!(miri) {
        for (k, &n) in sizes.iter().enumerate().skip(5) {
            assert!(n >= 500, "only {n} configurations near {} bytes", 1 << k);
        }
    }
}

#[test]
fn after_any_lie_the_walk_keeps_inside_and_gives_what_it_owes() {
    let mut told = [0u32; LIES.len()];
    for seed in 1..rounds(60_000) {
        let mut r = Rng::new(seed);
        let built = config(&mut r);
        let t = tell(&mut r, &built);
        let raw = t.descs.concat();
        let got: Vec<&[u8]> = walk(&raw).take(raw.len() + 1).collect();
        let mut at = 0;
        for d in &got {
            assert!(d.len() >= 2 && d[0] as usize == d.len(), "seed {seed}: {:?}", t.lie);
            assert!(at + d.len() <= raw.len(), "seed {seed}: past the buffer");
            assert_eq!(d.as_ptr(), raw[at..].as_ptr(), "seed {seed}: not the buffer's bytes");
            at += d.len();
        }
        let owed: Vec<&[u8]> = t.descs[..t.framed].iter().map(Vec::as_slice).collect();
        match t.ends {
            true => assert_eq!(got, owed, "seed {seed}: {:?}", t.lie),
            false => assert_eq!(got[..t.framed], owed[..], "seed {seed}: {:?}", t.lie),
        }
        told[LIES.iter().position(|l| *l == t.lie).unwrap()] += 1;
    }
    if !cfg!(miri) {
        for (n, lie) in told.iter().zip(LIES) {
            assert!(*n >= 3_000, "{lie:?} was told only {n} times");
        }
    }
}
