// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile endings drawn at random: the verdict on every look, and the end
//! of the wait wherever the final status lands.

use super::hostile::{expect, hostile_snap, rounds, seeded, xorshift, FATAL_IS, LINK, OFS};
use super::model_port::{rounds_of, ModelPort, Snap};
use crate::engine::completion::wait_done;
use crate::engine::verdict::{verdict, Verdict};
use crate::error::AhciError;

#[test]
fn the_verdict_matches_the_bits_on_every_look() {
    for seed in 1..rounds(200_000) {
        let mut s = seeded(seed);
        let snap = hostile_snap(&mut s);
        let want = match expect(snap) {
            Ok(()) => Verdict::Done,
            Err(AhciError::CommandFailed) => Verdict::Failed,
            Err(_) => Verdict::Pending,
        };
        let got = verdict(snap.ci, snap.sact, snap.is, snap.tfd);
        assert_eq!(got, want, "seed {seed}: {snap:?}");
        // With slot 0 outstanding, only PxIS or work never issued fails it.
        if got == Verdict::Failed && snap.ci & 1 != 0 {
            let named =
                snap.is & (FATAL_IS | OFS | LINK) != 0 || snap.ci & !1 != 0 || snap.sact != 0;
            assert!(named, "seed {seed}: failed on PxTFD alone with the slot set: {snap:?}");
        }
    }
}

#[test]
fn the_wait_ends_as_the_final_status_says_wherever_it_lands() {
    let mut reached = [0u64; 3];
    for seed in 1..rounds(100_000) {
        let mut s = seeded(seed);
        let after = hostile_snap(&mut s);
        let running_tfd = [0x80, 0x50, 0x51][(xorshift(&mut s) % 3) as usize];
        let before = Snap { ci: 1, sact: 0, is: xorshift(&mut s) as u32 & 0x7, tfd: running_tfd };
        let done_after = (xorshift(&mut s) % 40) as usize;
        let mut port = ModelPort::new(before, after, done_after);
        let got = wait_done(|off| port.read(off), rounds_of(64));
        assert_eq!(got, expect(after), "seed {seed}: {before:?} then {after:?} at {done_after}");
        reached[match got {
            Ok(()) => 0,
            Err(AhciError::CommandFailed) => 1,
            Err(_) => 2,
        }] += 1;
    }
    if !cfg!(miri) {
        assert!(reached.iter().all(|&n| n > 1_000), "outcomes reached: {reached:?}");
    }
}
