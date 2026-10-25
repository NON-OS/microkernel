// NONOS Operating System (AGPL-3.0-or-later)

//! Proofs for the slot a Wi-Fi scan or join comes back through from its
//! worker thread (libc's `Handoff`, the same file the capsule builds): one
//! request out at a time, the answer taken once by the window's tick, and an
//! answer that lands with no window looking (Settings closed) kept without
//! blocking the worker. The worker here is a real host thread.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use super::handoff_rig::*;
use crate::handoff::{Handoff, Look};

#[test]
fn a_scan_while_a_join_is_out_is_refused_until_the_join_answers() {
    let slot: Handoff<&'static str> = Handoff::new();
    assert!(slot.claim(), "the join goes out");
    assert!(!slot.claim(), "a scan now is refused, not queued behind it");
    assert_eq!(slot.poll(), Look::Waiting);
    slot.put("joined");
    assert!(!slot.claim(), "the answer not yet on the panel still holds the slot");
    assert_eq!(slot.poll(), Look::Ready("joined"));
    assert!(slot.claim(), "on the panel, the slot is free");
}

#[test]
fn the_tick_takes_the_workers_scan_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let slot = leaked::<Scanned>();
    assert!(slot.claim());
    let d = drops.clone();
    let worker = thread::spawn(move || slot.put(scanned(&[-40, -71, -63], &d)));
    let got = loop {
        match slot.poll() {
            Look::Ready(answer) => break answer,
            Look::Waiting => thread::yield_now(),
            Look::Idle => panic!("idle with a worker out"),
        }
    };
    worker.join().unwrap();
    assert_eq!(got, scanned(&[-40, -71, -63], &drops));
    assert_eq!(slot.poll(), Look::Idle, "handed over once");
}

#[test]
fn an_answer_with_no_window_looking_is_kept_and_does_not_hold_the_worker() {
    // Settings closed while the join was out: nothing polls. The worker's
    // put returns at once and the answer stays in the static slot.
    let drops = Arc::new(AtomicUsize::new(0));
    let slot = leaked::<Scanned>();
    assert!(slot.claim());
    let d = drops.clone();
    thread::spawn(move || slot.put(scanned(&[-50], &d))).join().unwrap();
    assert_eq!(drops.load(Ordering::SeqCst), 0, "kept, not lost");
    assert!(!slot.claim());
    // A window that looks later still finds it.
    assert_eq!(slot.poll(), Look::Ready(scanned(&[-50], &drops)));
}
