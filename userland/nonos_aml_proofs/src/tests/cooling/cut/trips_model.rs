// NONOS Operating System (AGPL-3.0-or-later)
//! What one read of the trips should give, from the runs it made (the log)
//! and the trips believed before, by the model in `mod.rs`.

use super::reader::{believed, Log};
use crate::namespace::Id;
use crate::thermal::{sane, Trips, UNREAD_AC, UNREAD_CRT, UNREAD_HOT, UNREAD_PSV, UNREAD_TZP};

/// The trips the model believes, the objects in `SEGS` order (None when
/// the zone has none).
pub fn want(log: &Log, ids: &[Option<Id>], prev: &Trips) -> Trips {
    let run = |i: usize| ids[i].map(|id| log.iter().find(|r| r.0 == id).map(|r| r.1));
    let mut bits = 0u16;
    let mut temp = |i: usize, was: Option<i32>, bit: u16| {
        let ran = run(i)?.expect("a trip there is read");
        believed(ran).or_else(|| {
            bits |= bit;
            was
        })
    };
    let crt = temp(1, prev.critical, UNREAD_CRT);
    let (hot, psv) = (temp(2, prev.hot, UNREAD_HOT), temp(3, prev.passive, UNREAD_PSV));
    let (hot, order) = sane(crt, hot, psv);
    // The levels fall from _AC0; one absent, warmer, or unread with none
    // kept ends them, and the ones after it are not read at all.
    let mut active: Vec<i32> = Vec::new();
    for x in 0..2 {
        match temp(4 + x, prev.active.get(x).copied(), UNREAD_AC) {
            Some(t) if active.last().is_none_or(|p| t <= *p) => active.push(t),
            _ => {
                assert!(x == 1 || run(5).is_none_or(|r| r.is_none()), "read past the end");
                break;
            }
        }
    }
    // _TZP is any integer; anything else keeps the period before.
    let polling = run(6).map(|r| match r.expect("_TZP read") {
        Some(Some(v)) => Some(v),
        _ => {
            bits |= UNREAD_TZP;
            prev.polling
        }
    });
    let polling = polling.flatten();
    Trips { critical: crt, hot, passive: psv, active, polling, faults: u16::from(order) | bits }
}
