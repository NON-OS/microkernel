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

//! Step 5, the replay: the log gives exactly the quoted PCRs 0, 1, 2, 4
//! and 7, event by event in order, and the last application in PCR 4 is
//! the release's loader.

use nsh_attest::consts::REPLAYED_PCRS;
use nsh_attest::Refusal;

use super::refused;
use crate::build::tcg::{EV_ACTION, EV_APP};
use crate::build::tcg_event::{digest, ev, note_ev};
use crate::build::tcg_log::{loader, Log, CALLING};
use crate::build::Spec;

/// The standard log's bytes with one event's bytes taken out or moved.
fn without(e: &[u8]) -> Vec<u8> {
    let log = Log::standard().bytes;
    let at = log.windows(e.len()).position(|w| w == e).expect("the event is in the log");
    [&log[..at], &log[at + e.len()..]].concat()
}

#[test]
fn a_quoted_value_the_log_does_not_give_is_refused() {
    for i in REPLAYED_PCRS {
        let s = Spec { pcr: vec![(i, [0x5E; 32])], ..Spec::default() };
        assert_eq!(refused(s), Refusal::Replay(i), "PCR {i}");
    }
}

#[test]
fn a_log_with_an_event_left_out_is_refused() {
    let mut s = Spec::default();
    s.log.bytes = without(&note_ev(4, EV_ACTION, CALLING));
    assert_eq!(refused(s), Refusal::Replay(4));
}

#[test]
fn a_log_with_its_events_out_of_order_is_refused() {
    let boot = ev(4, EV_APP, digest(b"boot manager"));
    let mut s = Spec::default();
    let mut bytes = without(&boot);
    let last = ev(4, EV_APP, loader());
    let at = bytes.windows(last.len()).position(|w| w == last).expect("loader");
    bytes.splice(at + last.len()..at + last.len(), boot);
    s.log.bytes = bytes;
    assert_eq!(refused(s), Refusal::Replay(4));
}
