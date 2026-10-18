// NONOS Operating System (AGPL-3.0-or-later)
//! The zone's watch over seeded runs of readings, real ones mixed with
//! ones cut or not believed (None), against the model after every reading.

use super::watch_model::{critical, pending, temp, AC0, CRT, HOT, PSV};
use crate::thermal::{Trips, Watch, CONFIRM_MS, CRITICAL_POLL_MS};

const POLL: u64 = 10_000;
const READS: [Option<i32>; 7] =
    [None, None, Some(40_000), Some(85_000), Some(95_000), Some(CRT), Some(120_000)];

fn zone() -> Trips {
    let t = (Some(CRT), Some(HOT), Some(PSV));
    Trips { critical: t.0, hot: t.1, passive: t.2, active: vec![AC0], ..Trips::default() }
}

#[test]
fn an_unread_reading_never_reads_cooler_and_crt_needs_two_real_ones() {
    let (t, mut settled, mut unread_hot) = (zone(), 0, 0);
    for seed in 0..400u64 {
        let (mut w, mut h, mut now, mut x) = (Watch::default(), Vec::new(), 0, seed);
        for _ in 0..40 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            now += [200, 500, 1_000, 1_500][(x >> 33) as usize % 4];
            let r = READS[(x >> 40) as usize % READS.len()];
            let was = w.clone();
            let wait = w.reading(&t, r, now, POLL);
            h.push((now, r));
            assert_eq!(w.unread, r.is_none());
            assert_eq!(w.temp, temp(&h), "seed {seed}: the last believed");
            assert_eq!(w.critical, critical(&h), "seed {seed} {h:?}");
            assert_eq!(w.critical_at.is_some(), temp(&h).is_some_and(|t| t >= CRT));
            let want = if pending(&h) {
                CONFIRM_MS
            } else if w.critical {
                CRITICAL_POLL_MS
            } else {
                POLL
            };
            assert_eq!(wait, want, "seed {seed}");
            if let Some(r) = r {
                let v = (w.verdict.critical, w.verdict.hot, w.verdict.passive);
                assert_eq!(v, (r >= CRT, r >= HOT, r >= PSV), "seed {seed}");
            } else {
                assert_eq!((&w.verdict, w.critical_at), (&was.verdict, was.critical_at));
                unread_hot += usize::from(was.verdict.hot);
            }
            settled += usize::from(w.critical && !was.critical);
        }
    }
    println!("watch: {settled} times _CRT settled, {unread_hot} unread readings while hot");
    assert!(settled >= 600 && unread_hot >= 2_100, "{settled} {unread_hot}");
}
