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

//! The caps, the loader's: a log over 1 MiB or 4096 events is refused whole,
//! never cut short.

use nsh_attest::consts::MAX_LOG;
use nsh_attest::log::{replay, MAX_EVENTS};
use nsh_attest::LogFault;

use crate::build::tcg::{header, SHA1, SHA256};
use crate::build::tcg_event::{event, extend};

fn events(n: usize) -> (Vec<u8>, [u8; 32]) {
    let (mut log, mut pcr) = (header(), [0u8; 32]);
    for i in 0..n {
        let d = nonos_hash::sha256(&(i as u32).to_be_bytes());
        log.extend(event(3, 1, &[(SHA1, vec![0; 20]), (SHA256, d.to_vec())], b""));
        pcr = extend(&pcr, &d);
    }
    (log, pcr)
}

#[test]
fn a_log_of_exactly_max_events_replays() {
    let (log, pcr3) = events(MAX_EVENTS);
    let r = replay(&log).expect("at the cap");
    assert_eq!((r.events, r.pcrs[3]), (MAX_EVENTS, pcr3));
}

#[test]
fn a_log_of_1025_events_is_refused() {
    assert_eq!(replay(&events(MAX_EVENTS + 1).0).map(|_| ()), Err(LogFault::TooManyEvents));
}

#[test]
fn a_log_over_the_byte_cap_is_refused() {
    let (mut log, _) = events(4);
    let d = [(SHA1, vec![0; 20]), (SHA256, vec![0; 32])];
    while MAX_LOG - log.len() > 72 + 60_000 {
        log.extend(event(5, 1, &d, &[0; 60_000]));
    }
    let pad = MAX_LOG - log.len() - 72;
    log.extend(event(5, 1, &d, &vec![0; pad]));
    assert_eq!(log.len(), MAX_LOG);
    assert!(replay(&log).is_ok(), "at the cap");
    log.push(0);
    assert_eq!(replay(&log).map(|_| ()), Err(LogFault::TooLarge));
}
