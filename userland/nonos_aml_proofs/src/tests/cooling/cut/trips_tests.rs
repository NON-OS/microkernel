// NONOS Operating System (AGPL-3.0-or-later)
//! Every scene's trips read with each method cut at every step below what
//! it used, against the model, from a first read and from one believed.

use super::fixture::{dsdt, scenes, SEGS};
use super::reader::{believed, held, limited, Log};
use crate::consts::at;
use crate::probe::Probe;
use crate::run::boot;
use crate::thermal::{sane, trips, Trips, UNREAD_CRT, UNREAD_HOT, UNREAD_PSV};

fn before() -> [Trips; 2] {
    let seen = (Some(95_050), Some(85_050), Some(75_050));
    let seen = Trips { critical: seen.0, hot: seen.1, passive: seen.2, ..Trips::default() };
    [Trips::default(), seen]
}

#[test]
fn a_cut_or_lying_trip_keeps_the_one_believed_before() {
    let (mut cut, mut kept) = (0, 0);
    for (lies, mutex) in scenes().into_iter().flat_map(|l| [(l, false), (l, true)]) {
        let (mut aml, _) = boot(&dsdt(&lies, mutex), 2);
        let tz = at(&aml, "\\_TZ.TZ0").unwrap();
        let ids: Vec<_> = SEGS.iter().map(|s| at(&aml, &format!("\\_TZ.TZ0.{s}"))).collect();
        for prev in before() {
            for steps in 1.. {
                let (mut log, mut probe) = (Log::new(), Probe::default());
                let t = trips(&mut aml, &mut limited(&mut probe, steps, &mut log), tz, &prev);
                assert!(!held(&aml), "{lies:?} {steps}: held");
                let mut bits = 0;
                let mut want = |i: usize, was: Option<i32>, bit: u8| {
                    let id = ids[i]?;
                    let run = log.iter().find(|r| r.0 == id).expect("each trip read").1;
                    believed(run).or_else(|| {
                        bits |= bit;
                        was
                    })
                };
                let crt = want(1, prev.critical, UNREAD_CRT);
                let (hot, psv) = (want(2, prev.hot, UNREAD_HOT), want(3, prev.passive, UNREAD_PSV));
                let (hot, order) = sane(crt, hot, psv);
                let got = (t.critical, t.hot, t.passive, t.faults);
                assert_eq!(got, (crt, hot, psv, order | bits), "{lies:?} {mutex} {steps}");
                assert_eq!((t.active, t.polling), (vec![70_050], Some(100)));
                let cuts = log.iter().filter(|r| r.1.is_none()).count();
                (cut, kept) =
                    (cut + cuts, kept + usize::from(bits != 0 && prev.critical.is_some()));
                if cuts == 0 {
                    break;
                }
            }
        }
    }
    println!("trips: {cut} cut reads, {kept} reads keeping a believed trip");
    assert!(cut >= 5_400 && kept >= 1_100, "{cut} {kept}");
}
