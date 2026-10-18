// NONOS Operating System (AGPL-3.0-or-later)
//! The SD host's stop over hosts that finish after any number of reads or
//! never, clocks of every pace, all-ones answers and lost windows.

use super::host::Host;
use super::sdhci_stop::{frees, reset_all, Outcome, BOUND_MS, MAX_READS};

#[test]
fn frames_are_freed_only_when_the_reset_was_seen_done() {
    let (mut stopped, mut running) = (0, 0);
    for step in [1, 2, 3, 7, 50, 99, 100, 101, 1_000] {
        for busy in (0..300).map(Some).chain([None]) {
            let mut h = Host { busy_reads: busy, step, ..Host::default() };
            let o = reset_all(&mut h);
            assert_eq!(h.writes, [(0x2F, 1)], "the reset asked once, nothing else");
            let waited = h.now - step; // from the first look
            match o {
                Outcome::Stopped => {
                    assert_eq!(h.last, Some(0), "seen done");
                    stopped += 1;
                }
                Outcome::StillRunning => {
                    assert_eq!(h.last, Some(1), "{step} {busy:?}: still busy when given up");
                    assert!(waited >= BOUND_MS, "{step} {busy:?}: gave up at {waited}");
                    running += 1;
                }
                Outcome::Unreachable => panic!("reachable"),
            }
            // Never waits on past the bound by more than one look.
            assert!(waited < BOUND_MS + step, "{step} {busy:?}: waited {waited}");
            assert_eq!(frees(o), o == Outcome::Stopped);
        }
    }
    println!("{stopped} stopped, {running} still running and kept");
    assert!(stopped >= 160 && running >= 2_000, "{stopped} {running}");
}

#[test]
fn a_frozen_clock_still_ends_and_keeps_the_frames() {
    let mut h = Host { busy_reads: None, step: 0, ..Host::default() };
    assert_eq!(reset_all(&mut h), Outcome::StillRunning);
    assert_eq!(h.reads, MAX_READS);
}

#[test]
fn all_ones_a_refused_write_or_a_lost_window_keep_the_frames() {
    let mut h = Host { busy_reads: Some(0), step: 1, all_ones: true, ..Host::default() };
    assert_eq!(reset_all(&mut h), Outcome::StillRunning, "a gone device reads all ones");
    let mut h = Host { busy_reads: Some(0), step: 1, refuse_write: true, ..Host::default() };
    assert_eq!(reset_all(&mut h), Outcome::Unreachable);
    assert_eq!(h.reads, 0, "nothing read after a refused write");
    for gone in 0..20 {
        let mut h =
            Host { busy_reads: Some(30), step: 1, gone_after: Some(gone), ..Host::default() };
        assert_eq!(reset_all(&mut h), Outcome::Unreachable, "gone after {gone}");
    }
    for o in [Outcome::StillRunning, Outcome::Unreachable] {
        assert!(!frees(o));
    }
}
