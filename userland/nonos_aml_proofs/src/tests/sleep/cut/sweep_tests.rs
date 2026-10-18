// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded shutdown prepared uncut, then with \_TTS or \_PTS cut at
//! every step below what it used, \_S5 given the full budget. The model:
//! the cut method fails out of steps and the other runs; each method's
//! markers are a prefix of the uncut run's; \_S5 is read every time, a
//! Name as it stands and a Method with what \_PTS stored (5 once its 20
//! marker shows, 0 with no \_PTS, either when cut before the marker); and
//! nothing \_PTS held is still held, so \_S5's wait for ever on the same
//! mutex returns at once. Seeds and floors are our choice, from no source.

use super::gen::scene;
use super::run::{group, run};
use crate::exec::FaultKind;
use crate::sleep::SleepTypes;

const SEEDS: u64 = 400;

#[test]
fn a_shutdown_cut_short_still_reads_s5() {
    // Cuts; cuts of a \_PTS that takes a mutex or \_GL; \_S5 methods read.
    let mut tally = [0usize; 3];
    for seed in 0..SEEDS {
        let sc = scene(seed);
        let (full, d, used, ..) = run(&sc, 1_000_000);
        assert!(
            full.tts.as_ref().is_none_or(|r| r.is_ok())
                && full.pts.as_ref().is_none_or(|r| r.is_ok())
        );
        let stored = if sc.pts.is_some() { 5 } else { 0 };
        for cut in 1..used {
            let (p, c, _, aml, probe) = run(&sc, cut);
            let what = format!("seed {seed} cut at {cut}");
            let (cut_r, other) = if sc.cut_pts { (&p.pts, &p.tts) } else { (&p.tts, &p.pts) };
            assert_eq!(
                (*cut_r).map(|r| r.map_err(|f| f.kind)),
                Some(Err(FaultKind::Steps)),
                "{what}"
            );
            assert!(other.as_ref().is_none_or(|r| r.is_ok()), "{what}");
            for tens in [1, 2, 3] {
                let (g, full_g) = (group(&c, tens), group(&d, tens));
                assert!(full_g.starts_with(&g), "{what}: {c:?} against {d:?}");
                assert!(tens != 3 || g == full_g, "{what}: \\_S5 ran");
            }
            let v = match (sc.s5_method, sc.cut_pts) {
                (false, _) => 3,
                (true, false) => stored,
                (true, true) if c.contains(&"Integer(20)".to_string()) => 5,
                (true, true) => p.types.map_or(9, |t| u64::from(t.a)),
            };
            assert!(v == 0 || v == 3 || v == 5, "{what}: \\_S5 read {v}");
            assert_eq!(p.types, Some(SleepTypes { a: v as u8, b: v as u8 }), "{what}");
            assert!(aml.rt.mutexes.is_empty() && aml.rt.global_lock == 0, "{what}: held");
            assert_eq!(probe.facs, Some(0), "{what}: the word");
            tally[0] += 1;
            tally[1] += usize::from(sc.cut_pts && sc.pts.is_some_and(|(_, w)| w > 0));
            tally[2] += usize::from(sc.s5_method);
        }
    }
    let floors = [9500, 3450, 6000];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
