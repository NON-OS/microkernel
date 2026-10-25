// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded scene run uncut against `model.rs`, then cut at every step
//! below what it used. A run cut after k markers stands between the
//! model's state after k operations and after k + 1. After every run:
//! no mutex is held, the Global Lock count is zero, the FACS word is free
//! unless firmware still holds it, GBL_RLS went only to firmware that
//! waited, and a later evaluation takes each mutex without waiting. Seeds
//! and floors are our choice, from no source.

use super::gen::scene;
use super::model::{uncut, Snap};
use super::run::{given_back, once, seen};
use crate::exec::FaultKind;

const SEEDS: u64 = 1500;

#[test]
fn whatever_is_held_at_a_cut_is_given_back() {
    // Cut runs; cuts with a mutex held; uncut runs done, faulted, timed out.
    let mut tally = [0usize; 5];
    for seed in 0..SEEDS {
        let sc = scene(seed);
        let want = uncut(&sc);
        let (got, used, mut r, left) = once(&sc, 1_000_000);
        assert_eq!(got, want.end, "seed {seed} {sc:?}");
        let k = want.snaps.len() - 1;
        let notes: Vec<u8> = r.aml.rt.notifications.iter().map(|n| n.1).collect();
        assert_eq!((notes, r.probe.debug.len()), (want.notes.clone(), k), "seed {seed}");
        let end = Snap { held: want.held_end, ..want.snaps[k] };
        assert_eq!(seen(&r, left), end, "seed {seed} {sc:?}");
        given_back(&mut r, &sc, &format!("seed {seed} uncut"));
        tally[2 + got.map_or(0, |f| if f == FaultKind::Time { 2 } else { 1 })] += 1;
        for steps in 1..used {
            let (cut, _, mut r, left) = once(&sc, steps);
            let what = format!("seed {seed} cut at {steps}");
            assert_eq!(cut, Some(FaultKind::Steps), "{what}");
            let k = r.probe.debug.len();
            let near = |j: usize| want.snaps.get(j).is_some_and(|s| *s == seen(&r, left));
            let held_mid = want.snaps.get(k).is_some_and(|s| s.held + 1 == left as usize);
            assert!(near(k) || near(k + 1) || held_mid, "{what}: {:?}", seen(&r, left));
            given_back(&mut r, &sc, &what);
            tally[0] += 1;
            tally[1] += usize::from(left > 0);
        }
    }
    let floors = [20_000, 15_600, 175, 975, 45];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
