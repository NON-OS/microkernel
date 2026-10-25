// NONOS Operating System (AGPL-3.0-or-later)

//! Every resend reaches an access point that took the first.

use nonos_wifi_core::rsn::JoinPolicy;

use super::beacons::beacon_for;
use super::radio::{clock, join_request};
use crate::assoc::{run_join, Radio};

// An access point that applies 802.11 duplicate detection (802.11-2020,
// 10.3.2.14.3): a frame with the Retry bit set whose sequence control matches
// the last one it took from this station is discarded. It takes the request but
// its answer never reaches the station, as when the answer is lost or is
// status 30 while it checks an earlier protected association.
struct DedupAp {
    last: Option<[u8; 2]>,
    taken: usize,
    discarded: usize,
}

impl Radio for DedupAp {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        let sc = [mpdu[22], mpdu[23]];
        if mpdu[1] & 0x08 != 0 && self.last == Some(sc) {
            self.discarded += 1;
        } else {
            self.taken += 1;
            self.last = Some(sc);
        }
        true
    }
    fn recv(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
}

// Every resend reaches an access point that took the first copy: none is
// discarded as a duplicate, so an answer lost once is asked for again.
#[test]
fn every_resend_reaches_an_access_point_that_took_the_first() {
    let mut ap = DedupAp { last: None, taken: 0, discarded: 0 };
    let mut now = clock(1);
    let report = run_join(&mut ap, &join_request(JoinPolicy::ANY), &beacon_for(b"net"), 1000, &mut now);
    assert_eq!(report.sent, 5);
    assert_eq!(ap.discarded, 0, "no resend is a duplicate of the first");
    assert_eq!(ap.taken, 5, "the access point took the first send and all four resends");
}
