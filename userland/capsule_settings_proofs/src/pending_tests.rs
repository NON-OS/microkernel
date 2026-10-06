// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the Wi-Fi panel's pending work: a scan or join asked for is
//! shown on the next tick and run on the one after, never both in one tick,
//! so "Scanning..." or "Joining..." is painted before the driver is asked;
//! it runs once; and a request no tick reached in time is dropped instead of
//! run long after.

use crate::wifi::pending::{Next, Pending, Work, STALE_MS};

#[test]
fn nothing_asked_does_nothing() {
    let mut p = Pending::Idle;
    assert_eq!(p.tick(0), Next::Nothing);
    assert_eq!(p.work(), None);
}

#[test]
fn a_scan_is_shown_then_run_once() {
    let mut p = Pending::ask(Work::Scan, 1_000);
    assert_eq!(p.work(), Some(Work::Scan));
    assert_eq!(p.tick(1_030), Next::Show);
    assert_eq!(p.work(), Some(Work::Scan), "still waiting while shown");
    assert_eq!(p.tick(1_060), Next::Run(Work::Scan));
    assert_eq!(p.work(), None);
    assert_eq!(p.tick(1_090), Next::Nothing);
}

#[test]
fn a_join_is_shown_then_run() {
    let mut p = Pending::ask(Work::Join, 0);
    assert_eq!(p.tick(30), Next::Show);
    assert_eq!(p.tick(60), Next::Run(Work::Join));
}

#[test]
fn asking_again_starts_over() {
    let mut p = Pending::ask(Work::Scan, 0);
    assert_eq!(p.tick(30), Next::Show);
    p = Pending::ask(Work::Join, 40);
    assert_eq!(p.tick(70), Next::Show);
    assert_eq!(p.tick(100), Next::Run(Work::Join));
}

#[test]
fn a_stale_request_is_dropped_not_run() {
    let mut p = Pending::ask(Work::Join, 0);
    assert_eq!(p.tick(STALE_MS), Next::Expired(Work::Join));
    assert_eq!(p.work(), None);
    let mut shown = Pending::ask(Work::Scan, 0);
    assert_eq!(shown.tick(10), Next::Show);
    assert_eq!(shown.tick(STALE_MS + 10), Next::Expired(Work::Scan));
}

#[test]
fn a_request_out_on_the_worker_stays_on_the_panel_and_is_never_dropped() {
    let mut p = Pending::ask(Work::Scan, 0);
    assert!(p.is_asked() && !p.is_out());
    assert_eq!(p.tick(30), Next::Show);
    assert_eq!(p.tick(60), Next::Run(Work::Scan));
    // The tick that ran it hands it to the worker.
    p = Pending::Out(Work::Scan);
    assert!(p.is_out() && !p.is_asked());
    assert_eq!(p.work(), Some(Work::Scan), "Scanning... stays up while the worker waits");
    // The driver's own budget ends it, not the stale rule: long after, a
    // request that is out is still waited for.
    assert_eq!(p.tick(STALE_MS * 2), Next::Nothing);
    let mut join = Pending::Out(Work::Join);
    assert_eq!(join.tick(i64::MAX), Next::Nothing);
    assert_eq!(join.work(), Some(Work::Join));
}
