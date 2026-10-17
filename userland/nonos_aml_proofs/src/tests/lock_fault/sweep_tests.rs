// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded scene run once without a step limit and then under every
//! step limit below what it used, so it is cut at each term in turn. The
//! model, stated apart: the uncut run ends as the scene says (firmware
//! holding for good, out of time; a select or index write fault, no
//! handler; a data fault, out of the region; else done), a cut run ends
//! out of steps unless it got that far; and after every run the capsule's
//! count is zero, the FACS word is free unless firmware still holds it,
//! and GBL_RLS was signalled only to firmware that waited. Seeds and
//! floors are our choice, from no source.

use super::fixture::source;
use super::gen::{scene, Scene};
use super::model::{given_back, uncut};
use crate::exec::{ExecLimits, FaultKind, Machine};
use crate::probe::Probe;
use crate::rig::Rig;

const SEEDS: u64 = 300;

fn once(sc: &Scene, steps: u64) -> (Option<FaultKind>, u64, Rig) {
    let facs = sc.holds.map(|h| if h > 0 { 2 } else { 0 });
    let probe = Probe {
        facs,
        firmware_holds: sc.holds.unwrap_or(0),
        firmware_waits: sc.waits,
        ..Default::default()
    };
    let mut r = Rig::with(&source(sc), probe);
    assert!(r.connect(0).is_empty() && r.connect(1).is_empty());
    let id = crate::consts::at(&r.aml, "\\M").unwrap();
    let lim = ExecLimits { steps, time_ms: 20, ..ExecLimits::DEFAULT };
    let mut m = Machine::new(&mut r.aml, &mut r.probe, lim);
    let got = m.evaluate(&mut r.aml, id, vec![]).err().map(|f| f.kind);
    let used = m.steps();
    (got, used, r)
}

#[test]
fn the_lock_is_given_back_however_the_access_ends() {
    // Cut runs; cut after a locked access began; uncut runs done, out of
    // time, and faulted in the select, index or data access.
    let mut tally = [0usize; 5];
    for seed in 0..SEEDS {
        let sc = scene(seed);
        let (got, used, r) = once(&sc, 1_000_000);
        assert_eq!(got, uncut(&sc), "seed {seed} {sc:?}");
        given_back(&r, &sc, &format!("seed {seed} uncut"));
        tally[2 + got.map_or(0, |k| if k == FaultKind::Time { 1 } else { 2 })] += 1;
        for steps in 1..used {
            let (cut, _, r) = once(&sc, steps);
            assert_eq!(cut, Some(FaultKind::Steps), "seed {seed} at {steps}");
            given_back(&r, &sc, &format!("seed {seed} cut at {steps}"));
            tally[0] += 1;
            tally[1] += usize::from(r.probe.facs.is_some() && !r.probe.log.is_empty());
        }
    }
    let floors = [3400, 520, 100, 50, 75];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
