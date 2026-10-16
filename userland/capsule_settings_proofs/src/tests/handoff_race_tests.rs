// NONOS Operating System (AGPL-3.0-or-later)

//! The hand-off's edges: a request no worker took gives its claim back, and
//! every answer is dropped exactly once when the window races it.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use super::handoff_rig::*;
use crate::handoff::{Handoff, Look};

#[test]
fn a_request_no_worker_took_gives_its_claim_back() {
    let slot: Handoff<u8> = Handoff::new();
    assert!(slot.claim());
    slot.release();
    assert_eq!(slot.poll(), Look::Idle);
    assert!(slot.claim(), "the window may ask again (or run it itself)");
}

#[test]
fn a_window_that_stops_waiting_races_the_answer_and_each_is_dropped_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let slot = leaked::<Scanned>();
    let rounds = 1_000usize;
    let mut shown = 0usize;
    for n in 0..rounds {
        assert!(slot.claim(), "round {n}");
        let d = drops.clone();
        let worker = thread::spawn(move || slot.put(scanned(&[n as i8], &d)));
        if n % 3 == 0 {
            slot.stop_waiting();
        }
        worker.join().unwrap();
        match slot.poll() {
            Look::Ready(answer) => {
                assert_eq!(answer.signals, vec![n as i8], "only this round's answer");
                assert!(n % 3 != 0);
                shown += 1;
            }
            Look::Idle => assert!(n % 3 == 0),
            Look::Waiting => panic!("round {n}: the worker has finished"),
        }
    }
    assert_eq!(shown, rounds - rounds.div_ceil(3));
    assert_eq!(drops.load(Ordering::SeqCst), rounds);
}
