// NONOS Operating System (AGPL-3.0-or-later)
//! F-111: the order a shutdown's methods run in, for every mix of \_TTS and
//! \_PTS present, absent or faulting, and \_S5 absent, a Name, or a method
//! that reads what \_PTS stored. The model (ACPI 6.5, 7.4 and 16.1;
//! recalled, unverified): \_TTS, then \_PTS, then \_S5, each only if
//! present; a fault in \_TTS or \_PTS stops neither the rest nor \_S5; and
//! \_S5 is evaluated last, so a value \_PTS computes is the one written.

use crate::enc::*;
use crate::probe::Probe;
use crate::run::boot;
use crate::sleep::{prepare, SleepTypes};
use crate::tests::platform::run::evaluator;

const ABSENT: u8 = 0;
const RUNS: u8 = 1;
const FAULTS: u8 = 2;

fn method_of(path: &str, tag: u64, how: u8, also: &[u8]) -> Vec<u8> {
    let fatal = cat(&[&op(0x5B32), &[1], &7u32.to_le_bytes(), &int(0)]);
    let tail: &[u8] = if how == FAULTS { &fatal } else { &[] };
    match how {
        ABSENT => Vec::new(),
        _ => method(path, 1, &cat(&[&store(&int(tag), &op(0x5B31)), also, tail])),
    }
}

/// \_S5 as a method: says it ran, then fills a package from what \_PTS
/// stored (a package literal of names would hold references, not values).
fn s5_body() -> Vec<u8> {
    let at = |i: u64| e(0x88, &[&name("\\SPKG"), &int(i), &[0]]);
    let fill = cat(&[&store(&name("\\PTSV"), &at(0)), &store(&name("\\PTSV"), &at(1))]);
    cat(&[&store(&int(3), &op(0x5B31)), &fill, &ret(&name("\\SPKG"))])
}

#[test]
fn tts_then_pts_then_s5_whatever_fails() {
    for tts in [ABSENT, RUNS, FAULTS] {
        for pts in [ABSENT, RUNS, FAULTS] {
            for s5 in [ABSENT, 1, 2] {
                let keep = store(&arg(0), &name("\\PTSV"));
                let s5_src = match s5 {
                    1 => name_obj("\\_S5", &package(2, &[int(3), int(3)])),
                    2 => method("\\_S5", 0, &s5_body()),
                    _ => Vec::new(),
                };
                let src = cat(&[
                    &name_obj("\\PTSV", &int(0)),
                    &name_obj("\\SPKG", &package(2, &[int(0), int(0)])),
                    &method_of("\\_TTS", 1, tts, &[]),
                    &method_of("\\_PTS", 2, pts, &keep),
                    &s5_src,
                ]);
                let (mut aml, _) = boot(&src, 2);
                let mut probe = Probe::default();
                let p = prepare(&mut aml, &mut evaluator(&mut probe), 5);
                let mut order = Vec::new();
                order.extend((tts != ABSENT).then_some("Integer(1)"));
                order.extend((pts != ABSENT).then_some("Integer(2)"));
                order.extend((s5 == 2).then_some("Integer(3)"));
                assert_eq!(probe.debug, order, "tts {tts} pts {pts} s5 {s5}");
                let v = if pts != ABSENT { 5 } else { 0 };
                let want = [None, Some(SleepTypes { a: 3, b: 3 }), Some(SleepTypes { a: v, b: v })];
                assert_eq!(p.types, want[s5 as usize], "tts {tts} pts {pts} s5 {s5}");
                assert_eq!(p.tts.map(|r| r.is_ok()), (tts != ABSENT).then_some(tts == RUNS));
                assert_eq!(p.pts.map(|r| r.is_ok()), (pts != ABSENT).then_some(pts == RUNS));
            }
        }
    }
}
