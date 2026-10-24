// NONOS Operating System (AGPL-3.0-or-later)
//! The PACKET_MSG walk over seeded bulk IN transfers, built sound and then
//! told one lie: it takes exactly the frames and drops exactly the
//! messages the rule in batch_sweep/model.rs does, and exactly the honest
//! frames the lie leaves, each inside the transfer and an Ethernet frame.

use std::collections::VecDeque;

use crate::batch_sweep::build::transfer;
use crate::batch_sweep::lie::{tell, LIES};
use crate::batch_sweep::model;
use crate::batch_sweep::rng::{rounds, Rng};
use crate::rndis::batch::frames;

fn driver(t: &[u8]) -> (Vec<std::ops::Range<usize>>, u32) {
    let mut q = VecDeque::new();
    let dropped = frames(t, &mut q);
    (q.into_iter().collect(), dropped)
}

#[test]
fn every_sound_transfer_gives_every_frame_and_drops_none() {
    for seed in 1..rounds(30_000) {
        let t = transfer(&mut Rng::new(seed));
        let all = (t.msgs.iter().map(|m| m.1.clone()).collect(), 0);
        assert_eq!(driver(&t.raw), all, "seed {seed}");
        assert_eq!(model::frames(&t.raw), all, "seed {seed}: the rule");
    }
}

#[test]
fn after_any_lie_the_walk_takes_exactly_the_honest_frames_left() {
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(60_000) {
        let x = tell(&mut Rng::new(seed));
        let got = driver(&x.t.raw);
        let why = format!("seed {seed}: {:?}", x.lie);
        assert_eq!(got, model::frames(&x.t.raw), "{why}");
        assert_eq!(got, x.expect, "{why}: the honest frames left");
        for f in &got.0 {
            assert!(f.end <= x.t.raw.len() && (14..=1514).contains(&f.len()), "{why}: {f:?}");
        }
        told[LIES.iter().position(|l| *l == x.lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 3_000, "{lie:?} was told only {n} times");
    }
}
