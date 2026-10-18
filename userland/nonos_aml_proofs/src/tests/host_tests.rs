// NONOS Operating System (AGPL-3.0-or-later)
//! What reaches the host: Fatal, Timer, Sleep, Stall and Debug.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::run::{boot, call_with, Probe};

#[test]
fn fatal_timer_sleep_and_stall_reach_the_host() {
    let body = cat(&[
        &e(0x5B22, &[&int(3)]),
        &e(0x5B21, &[&int(50)]),
        &store(&op(0x5B33), &local(0)),
        &e(0x5B32, &[&[1], &[0x34, 0x12, 0, 0], &int(7)]),
    ]);
    let (mut aml, _) = boot(&method("\\M", 0, &body), 2);
    let mut probe = Probe::default();
    let r = call_with(&mut aml, &mut probe, Default::default(), "\\M", vec![]);
    assert_eq!(r.unwrap_err().kind, FaultKind::Fatal);
    assert_eq!((probe.slept_ms, probe.stalled_us), (vec![3], vec![50]));
    assert_eq!(probe.fatal, [(1, 0x1234, 7)]);
    assert_eq!(probe.ticks, 10);
}
