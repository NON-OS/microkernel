// NONOS Operating System (AGPL-3.0-or-later)

//! A NIC never ready, no ALIVE, a dead ALIVE status: each bounded.

use super::rig::{polls, rig, run_boot};
use crate::gen3::bringup::BootError;
use crate::gen3::start::StartError;

#[test]
fn a_nic_that_never_reports_ready_stops_the_boot() {
    let r = rig();
    r.model.s.borrow_mut().ready_never = true;
    let mut c = polls();
    assert_eq!(run_boot(&r, &mut c), Err(BootError::Start(StartError::NotReady)));
}

#[test]
fn no_alive_cause_is_a_bounded_failure() {
    let r = rig();
    r.model.s.borrow_mut().no_alive = true;
    let mut c = polls();
    assert_eq!(run_boot(&r, &mut c), Err(BootError::NoAliveCause));
}

#[test]
fn a_dead_alive_status_is_refused() {
    let r = rig();
    r.model.s.borrow_mut().alive_status = 0xDEAD;
    let mut c = polls();
    assert_eq!(run_boot(&r, &mut c), Err(BootError::NotAlive(0xDEAD)));
}
