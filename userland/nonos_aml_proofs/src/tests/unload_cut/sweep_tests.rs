// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded scene run uncut, then cut at every step below what it
//! used. After every run: \D1's objects are all there or all gone; no
//! mutex is held; every event, notification, region and _REG mark the
//! runtime keeps names a live object; a later evaluation takes \D1's mutex
//! at once if \D1 is there; after a reload the mutex was free and the
//! event unsignalled; and, uncut, a reference kept in a Local past the
//! unload reads as stale, reload or not. Seeds and floors are our choice, from no source.

use super::fixture::dsdt;
use super::gen::{scene, Scene, Use};
use crate::exec::{ExecLimits, FaultKind, Machine};
use crate::rig::Rig;

const PARTS: [&str; 6] = ["\\D1", "\\D1.MTX", "\\D1.EVT", "\\D1.R", "\\D1.F1", "\\D1._REG"];

fn run(sc: &Scene, steps: u64) -> (Option<FaultKind>, u64, Rig) {
    let mut r = Rig::new(&dsdt(sc));
    assert!(r.connect(1).is_empty());
    let id = crate::consts::at(&r.aml, "\\M").unwrap();
    let mut m = Machine::new(&mut r.aml, &mut r.probe, ExecLimits { steps, ..ExecLimits::DEFAULT });
    let end = m.evaluate(&mut r.aml, id, vec![]).err().map(|f| f.kind);
    let used = m.steps();
    (end, used, r)
}

fn check(r: &mut Rig, what: &str) -> [bool; 2] {
    let here: Vec<bool> = PARTS.iter().map(|p| crate::consts::at(&r.aml, p).is_some()).collect();
    assert!(here.iter().all(|h| *h == here[0]), "{what}: {here:?}");
    let rt = &r.aml.rt;
    assert!(rt.mutexes.is_empty(), "{what}: a mutex held");
    let live = |id: &u32| r.aml.ns.get(*id).is_some();
    assert!(rt.events.keys().all(live) && rt.regions.keys().all(live), "{what}: events or regions");
    assert!(rt.notifications.iter().all(|n| live(&n.0)), "{what}: a notification");
    assert!(rt.announced.iter().all(|a| live(&a.0)), "{what}: a _REG mark");
    if here[0] {
        assert_eq!(r.num("\\TRY"), 0, "{what}: the mutex taken at once");
    }
    let said: Vec<bool> =
        (0..5).map(|n| r.probe.debug.contains(&format!("Integer({n})"))).collect();
    let unloaded = said[3];
    if said[4] {
        assert_eq!(r.num("\\R1"), 0, "{what}: reload's mutex free");
        assert_eq!(r.num("\\R2"), u64::MAX, "{what}: reload's event unsignalled");
    }
    [unloaded, said[4]]
}

#[test]
fn an_unload_in_use_leaves_nothing_behind_at_any_cut() {
    // Cuts; cuts after the unload; stale even past a reload; reloads checked.
    let mut tally = [0usize; 4];
    for seed in 0..400 {
        let sc = scene(seed);
        let (end, used, mut r) = run(&sc, 1_000_000);
        let keep = sc.uses.iter().any(|u| matches!(u, Use::Keep));
        let want = match (keep, sc.reload) {
            (true, _) => Some(FaultKind::Stale),
            (false, true) => Some(FaultKind::Mutex),
            _ => None,
        };
        assert_eq!(end, want, "seed {seed} {sc:?}: stale reference, or reloaded mutex fresh");
        check(&mut r, &format!("seed {seed} uncut"));
        tally[2] += usize::from(keep && sc.reload);
        for steps in 1..used {
            let (_, _, mut r) = run(&sc, steps);
            let [unloaded, reloaded] = check(&mut r, &format!("seed {seed} cut at {steps}"));
            tally[0] += 1;
            tally[1] += usize::from(unloaded);
            tally[3] += usize::from(reloaded);
        }
    }
    let floors = [13_200, 4_450, 75, 960];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
