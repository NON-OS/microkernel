// NONOS Operating System (AGPL-3.0-or-later)
//! F-110b, the the ruling on misordered trip points, as a table, as a
//! seeded sweep against a model, and through the interpreter. The model:
//! _CRT is never touched; _HOT at or above _CRT is dropped; _HOT at or
//! below _PSV is dropped while a _CRT above _PSV stands, else kept; a _CRT
//! at or below _PSV is kept; each case is a fault bit. Dropping a _HOT
//! never leaves the zone without its _CRT.

use crate::consts::at;
use crate::enc::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;
use crate::thermal::{sane, trips, CRT_UNDER_PSV as CL, HOT_OVER_CRT as HO};
use crate::thermal::{HOT_UNDER_PSV as HU, HOT_UNDER_PSV_KEPT as HK};

fn model(c: Option<i32>, h: Option<i32>, p: Option<i32>) -> (Option<i32>, u8) {
    let low = |t: Option<i32>| matches!((t, p), (Some(t), Some(p)) if t <= p);
    let mut f = if low(c) { CL } else { 0 };
    let kept = match h {
        Some(h) if c.is_some_and(|c| h >= c) => (f |= HO, None).1,
        Some(_) if low(h) && c.is_some() && !low(c) => (f |= HU, None).1,
        Some(_) if low(h) => (f |= HK, h).1,
        _ => h,
    };
    (kept, f)
}

#[test]
fn the_table() {
    let s = |c, h, p| sane(Some(c), Some(h), Some(p));
    assert_eq!(s(100, 90, 80), (Some(90), 0), "in order");
    assert_eq!(s(100, 100, 80), (None, HO), "_HOT at _CRT");
    assert_eq!(s(100, 70, 80), (None, HU), "_HOT under _PSV, _CRT guards");
    assert_eq!(s(70, 60, 80), (Some(60), HK | CL), "no _CRT above _PSV: _HOT kept");
    assert_eq!(s(80, 90, 80), (None, HO | CL), "_CRT at _PSV, kept; _HOT over it");
    assert_eq!(sane(None, Some(70), Some(80)), (Some(70), HK), "no _CRT at all");
    assert_eq!(sane(Some(100), Some(70), None), (Some(70), 0), "no _PSV to order by");
    assert_eq!(sane(Some(60), None, Some(80)), (None, CL));
}

#[test]
fn every_ordering_is_its_model() {
    let s = &mut 0x9E37_79B9_7F4A_7C15u64;
    let mut seen = 0u8;
    for _ in 0..20_000 {
        let mut pick = || {
            *s ^= *s << 13;
            *s ^= *s >> 7;
            *s ^= *s << 17;
            (!s.is_multiple_of(5)).then_some(40_000 + (*s % 7) as i32 * 5_000)
        };
        let (c, h, p) = (pick(), pick(), pick());
        let got = sane(c, h, p);
        assert_eq!(got, model(c, h, p), "{c:?} {h:?} {p:?}");
        assert!(h.is_none() || got.0.is_some() || c.is_some(), "a dropped _HOT leaves its _CRT");
        seen |= got.1;
    }
    assert_eq!(seen, HO | HU | HK | CL, "every fault met");
}

#[test]
fn trips_applies_it() {
    let trip = |seg: &str, dk: u64| name_obj(seg, &int(dk));
    let body = cat(&[&trip("_CRT", 3532), &trip("_HOT", 3632), &trip("_PSV", 3582)]);
    let (mut aml, _) = boot(&thermal_zone("\\_TZ.TZ9", &body), 2);
    let tz = at(&aml, "\\_TZ.TZ9").unwrap();
    let t = trips(&mut aml, &mut evaluator(&mut Probe::default()), tz, &Default::default());
    assert_eq!((t.critical, t.hot, t.faults), (Some(80_050), None, u16::from(HO | CL)));
}
