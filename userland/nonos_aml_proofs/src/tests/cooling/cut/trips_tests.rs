// NONOS Operating System (AGPL-3.0-or-later)
//! Every scene's trips read with each method cut at every step below what
//! it used, against the model, from a first read and from one believed.

use super::fixture::{dsdt, scenes, SEGS};
use super::reader::{held, limited, Log};
use super::trips_model::want;
use crate::consts::at;
use crate::probe::Probe;
use crate::run::boot;
use crate::thermal::{trips, Trips, UNREAD_AC, UNREAD_TRIPS, UNREAD_TZP};
use crate::thermal::{UNREAD_CRT as C, UNREAD_HOT as H, UNREAD_PSV as P};

fn before() -> [Trips; 2] {
    let seen = (Some(95_050), Some(85_050), Some(75_050));
    let (critical, hot, passive) = seen;
    let more = (vec![65_050, 55_050], Some(50));
    let seen = Trips { critical, hot, passive, active: more.0, polling: more.1, faults: 0 };
    [Trips::default(), seen]
}

#[test]
fn a_cut_or_lying_trip_keeps_the_one_believed_before() {
    let (mut cut, mut kept, mut more) = (0, 0, [0; 2]);
    for (lies, mutex) in scenes().into_iter().flat_map(|l| [(l, false), (l, true)]) {
        let (mut aml, _) = boot(&dsdt(&lies, mutex), 2);
        let tz = at(&aml, "\\_TZ.TZ0").unwrap();
        let ids: Vec<_> = SEGS.iter().map(|s| at(&aml, &format!("\\_TZ.TZ0.{s}"))).collect();
        for prev in before() {
            for steps in 1.. {
                let (mut log, mut probe) = (Log::new(), Probe::default());
                let t = trips(&mut aml, &mut limited(&mut probe, steps, &mut log), tz, &prev);
                assert!(!held(&aml), "{lies:?} {steps}: held");
                assert_eq!(t, want(&log, &ids, &prev), "{lies:?} {mutex} {steps}");
                let cuts = log.iter().filter(|r| r.1.is_none()).count();
                cut += cuts;
                let seen = prev.critical.is_some();
                kept += usize::from(t.faults >> 4 != 0 && seen);
                for (n, bit) in more.iter_mut().zip([UNREAD_AC, UNREAD_TZP]) {
                    *n += usize::from(t.faults & bit != 0 && seen);
                }
                if cuts == 0 {
                    break;
                }
            }
        }
    }
    println!("trips: {cut} cut reads, {kept} keeping a trip, {more:?} an _ACx, a _TZP");
    assert!(cut >= 22_500 && kept >= 3_000, "{cut} {kept}");
    assert!(more[0] >= 2_700 && more[1] >= 2_900, "{more:?}");
}

/// The capsule reads a zone's trips again while any of these is set: every
/// trip whose unread value is kept must be among them.
#[test]
fn every_kept_trip_is_read_again() {
    assert_eq!(UNREAD_TRIPS, C | H | P | UNREAD_AC | UNREAD_TZP);
}
