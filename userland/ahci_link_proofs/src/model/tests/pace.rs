// NONOS Operating System (AGPL-3.0-or-later)
//! Every wait sleeps between its looks once it has spun a moment, so a slow
//! drive does not hold a core for the whole of its budget.

use crate::clock::{idles, wait_until};
use crate::engine::transfer::transfer;
use crate::model::{Outcome, Rig, Step};

#[test]
fn a_long_wait_sleeps_between_looks() {
    let _rig = Rig::new(Vec::new(), true);
    let before = idles();
    assert!(!wait_until(100, || false));
    assert!(idles() - before >= 50, "{} sleeps in 100 ms", idles() - before);
}

#[test]
fn a_slow_command_sleeps_between_looks() {
    let mut rig = Rig::new(vec![Step { ms: 200, outcome: Outcome::Clean }], true);
    let regs = rig.regs();
    let before = idles();
    assert_eq!(transfer(&mut rig.port, regs, 3, 2, false), Ok(()));
    assert!(idles() - before >= 100, "{} sleeps in 200 ms", idles() - before);
}
