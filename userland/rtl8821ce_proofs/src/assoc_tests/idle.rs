// NONOS Operating System (AGPL-3.0-or-later)

//! A join never spins: every pass in which the radio had nothing ends in the
//! clock's pause, and a pass that heard a frame does not pause.

use nonos_wifi_core::rsn::JoinPolicy;

use super::beacons::beacon_for;
use super::radio::join_request;
use crate::assoc::{run_join, JoinClock, Outcome, Radio};

// A radio that hears one stray frame every third poll and nothing else.
struct Sparse {
    polls: u32,
    empty: u32,
}

impl Radio for Sparse {
    fn send(&mut self, _: &[u8]) -> bool {
        true
    }
    fn recv(&mut self, out: &mut [u8]) -> Option<usize> {
        self.polls += 1;
        if self.polls.is_multiple_of(3) {
            out[..4].copy_from_slice(&[0x80, 0, 0, 0]);
            return Some(4);
        }
        self.empty += 1;
        None
    }
}

// A clock a millisecond on per reading, counting its pauses.
struct Counting {
    now: u64,
    idles: u32,
}

impl JoinClock for Counting {
    fn now_ms(&mut self) -> u64 {
        self.now += 1;
        self.now
    }
    fn idle(&mut self) {
        self.idles += 1;
    }
}

#[test]
fn a_join_pauses_on_every_empty_poll_and_only_then() {
    let mut radio = Sparse { polls: 0, empty: 0 };
    let mut clock = Counting { now: 0, idles: 0 };
    let req = join_request(JoinPolicy::ANY);
    let report = run_join(&mut radio, &req, &beacon_for(b"net"), 60, &mut clock);
    assert!(matches!(report.outcome, Outcome::TimedOut));
    assert!(radio.empty > 10, "the radio was mostly quiet");
    assert_eq!(clock.idles, radio.empty, "one pause per empty poll");
}
