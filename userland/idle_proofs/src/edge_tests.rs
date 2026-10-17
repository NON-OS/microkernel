// NONOS Operating System (AGPL-3.0-or-later)
//! `pick` at the edges its rule names: the deepest taken past those it may
//! not enter, the budget to the microsecond, and only states it can record.

use crate::arch::fdt::find::idle::{IdleState, MAX_STATES};

const NONE: Allow = Allow { powerdown: false, timer_stop: false };
const DOWN: Allow = Allow { powerdown: true, timer_stop: false };
const ALL: Allow = Allow { powerdown: true, timer_stop: true };
use crate::rules::Allow;
use crate::select::pick;

#[test]
fn the_edges() {
    let s = |param, res, stops| IdleState {
        param,
        exit_us: 5,
        residency_us: res,
        timer_stops: stops,
        ..Default::default()
    };
    let states = [s(0, 100, false), s(0, 900, true), s(1 << 16, 800, false), s(0, 300, false)];
    let all = [0, 1, 2, 3];
    // The deepest states stop the timer or power down: the next one is taken.
    assert_eq!(pick(&states, &all, false, 0, u64::MAX, NONE), Some(3));
    assert_eq!(pick(&states, &all, false, 1 << 3, u64::MAX, NONE), Some(0));
    assert_eq!(pick(&states, &all, false, 1 | 1 << 3, u64::MAX, NONE), None);
    // With the resume path, the powerdown state is the deepest that may be.
    assert_eq!(pick(&states, &all, false, 0, u64::MAX, DOWN), Some(2));
    assert_eq!(pick(&states, &all, true, 0, u64::MAX, DOWN), Some(2));
    // With a broadcast frame too, the timer-stop state is the deepest.
    assert_eq!(pick(&states, &all, false, 0, u64::MAX, ALL), Some(1));
    let stop = Allow { timer_stop: true, ..NONE };
    assert_eq!(pick(&states, &all, false, 0, u64::MAX, stop), Some(1));
    // Residency and exit together fit the budget, to the microsecond.
    assert_eq!(pick(&states, &all, false, 0, 305, NONE), Some(3));
    assert_eq!(pick(&states, &all, false, 0, 304, NONE), Some(0));
    assert_eq!(pick(&states, &all, false, 0, 104, NONE), None);
    // Only this CPU's states, and none it cannot name or record.
    assert_eq!(pick(&states, &[0], false, 0, u64::MAX, NONE), Some(0));
    assert_eq!(pick(&states, &[], false, 0, u64::MAX, NONE), None);
    let many = [s(0, 1, false); MAX_STATES + 2];
    assert_eq!(pick(&many, &[8, 9, 200], false, 0, u64::MAX, NONE), None);
    let huge = [s(0, u32::MAX, false)];
    assert_eq!(pick(&huge, &[0], false, 0, u64::MAX, NONE), Some(0));
}
