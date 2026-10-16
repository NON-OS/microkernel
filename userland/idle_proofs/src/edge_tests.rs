// NONOS Operating System (AGPL-3.0-or-later)
//! `pick` at the edges its rule names: the deepest taken past those it may
//! not enter, the budget to the microsecond, and only states it can record.

use crate::arch::fdt::find::idle::{IdleState, MAX_STATES};
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
    assert_eq!(pick(&states, &all, false, 0, u64::MAX), Some(3));
    assert_eq!(pick(&states, &all, false, 1 << 3, u64::MAX), Some(0));
    assert_eq!(pick(&states, &all, false, 1 | 1 << 3, u64::MAX), None);
    // Residency and exit together fit the budget, to the microsecond.
    assert_eq!(pick(&states, &all, false, 0, 305), Some(3));
    assert_eq!(pick(&states, &all, false, 0, 304), Some(0));
    assert_eq!(pick(&states, &all, false, 0, 104), None);
    // Only this CPU's states, and none it cannot name or record.
    assert_eq!(pick(&states, &[0], false, 0, u64::MAX), Some(0));
    assert_eq!(pick(&states, &[], false, 0, u64::MAX), None);
    let many = [s(0, 1, false); MAX_STATES + 2];
    assert_eq!(pick(&many, &[8, 9, 200], false, 0, u64::MAX), None);
    let huge = [s(0, u32::MAX, false)];
    assert_eq!(pick(&huge, &[0], false, 0, u64::MAX), Some(0));
}
