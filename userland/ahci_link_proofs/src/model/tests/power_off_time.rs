// NONOS Operating System (AGPL-3.0-or-later)
//! The power-off's time on model time: a flush or a standby that never
//! ends still answers inside the kernel's wait, and a standby the disk
//! refuses is the reply.

use super::power_off::run;
use crate::clock::{now, run_on_model_time};
use crate::constants::power_off::{standby_budget, KERNEL_POWER_OFF_MS};
use crate::error::AhciError;
use crate::model::{Outcome, Rig, Step};
use crate::power_off_rule::Standby;

#[test]
fn a_flush_that_never_ends_answers_inside_the_kernels_wait() {
    run_on_model_time(true);
    let late = Step { ms: 40_000, outcome: Outcome::Clean };
    let mut rig = Rig::new(vec![late], true);
    let start = now();
    let done = run(&mut rig, true);
    let spent = now().duration_since(start).as_millis() as u64;
    run_on_model_time(false);
    assert_eq!(done.flushed, Err(AhciError::Timeout));
    assert_eq!(done.standby, Standby::NoTime, "no standby fits after the flush's longest");
    assert_eq!(done.failure(), Err(AhciError::Timeout), "the flush's failure is the reply");
    assert!(spent < KERNEL_POWER_OFF_MS, "{spent} ms, past the kernel's wait");
}

#[test]
fn a_standby_that_never_ends_answers_inside_the_kernels_wait() {
    run_on_model_time(true);
    let clean = Step { ms: 1, outcome: Outcome::Clean };
    let late = Step { ms: 40_000, outcome: Outcome::Clean };
    let mut rig = Rig::new(vec![clean, late], true);
    let start = now();
    let done = run(&mut rig, true);
    let spent = now().duration_since(start).as_millis() as u64;
    run_on_model_time(false);
    assert_eq!((done.flushed, done.standby), (Ok(()), Standby::Failed(AhciError::Timeout)));
    assert_eq!(rig.port.command_ms, standby_budget(true).unwrap(), "the standby's own budget");
    assert!(spent < KERNEL_POWER_OFF_MS, "{spent} ms, past the kernel's wait");
}

#[test]
fn a_standby_the_disk_refuses_is_the_reply() {
    let refuse = Step { ms: 1, outcome: Outcome::Error(0x04) };
    let mut rig = Rig::new(vec![Step { ms: 1, outcome: Outcome::Clean }, refuse], true);
    let done = run(&mut rig, true);
    assert_eq!(done.flushed, Ok(()));
    assert!(matches!(done.standby, Standby::Failed(_)));
    assert!(done.failure().is_err());
}
