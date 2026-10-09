// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::log_build::{digest, ev, event, standard, EV_APP, SHA1, SHA256, SHA384};
use crate::tcg::{replay, LogError, MAX_EVENTS, MAX_EVENT_BYTES, MAX_LOG_BYTES};

fn log_of(events: &[Vec<u8>]) -> Vec<u8> {
    let mut log = standard();
    events.iter().for_each(|e| log.extend(e));
    log
}

#[test]
fn digests_under_undeclared_repeated_or_too_many_banks_are_refused() {
    let d = digest(1).to_vec();
    let bad = |ds: &[(u16, Vec<u8>)]| replay(&log_of(&[event(4, EV_APP, ds, b"")]));
    assert_eq!(bad(&[(SHA384, vec![0; 48])]), Err(LogError::UnknownBank));
    assert_eq!(bad(&[(SHA256, d.clone()), (SHA256, d.clone())]), Err(LogError::UnknownBank));
    let three = [(SHA1, vec![0; 20]), (SHA256, d.clone()), (SHA1, vec![0; 20])];
    assert_eq!(bad(&three), Err(LogError::TooManyBanks));
    assert_eq!(bad(&[(SHA1, vec![0; 20])]), Err(LogError::MissingSha256));
    assert!(replay(&log_of(&[event(7, 1, &[(SHA1, vec![0; 20])], b"")])).is_ok());
}

#[test]
fn every_bound_holds() {
    let big = event(4, EV_APP, &[(SHA256, digest(1).to_vec())], &vec![0; MAX_EVENT_BYTES + 1]);
    assert_eq!(replay(&log_of(&[big])), Err(LogError::EventTooLarge));
    let one = ev(5, 1, digest(1));
    let many = log_of(&vec![one; MAX_EVENTS + 1]);
    assert_eq!(replay(&many), Err(LogError::TooManyEvents));
    assert_eq!(replay(&vec![0; MAX_LOG_BYTES + 1]), Err(LogError::TooLarge));
}

/// Every single-bit flip of a real-shaped log: never a panic, and a flip in the
/// PCR 4 event's SHA-256 digest never replays to the honest PCR.
#[test]
fn no_flip_of_any_bit_panics_and_a_digest_flip_moves_the_pcr() {
    let first = ev(0, 1, digest(9));
    let log = log_of(&[first.clone(), ev(4, EV_APP, digest(1)), ev(7, 6, digest(3))]);
    let honest = replay(&log).expect("replays").pcr4;
    let at = standard().len() + first.len() + 12 + 2 + 20 + 2;
    for bit in 0..log.len() * 8 {
        let mut m = log.clone();
        m[bit / 8] ^= 1 << (bit % 8);
        let moved = replay(&m).map_or(true, |r| r.pcr4 != honest);
        assert!(moved || !(at..at + 32).contains(&(bit / 8)), "bit {bit}");
    }
}
