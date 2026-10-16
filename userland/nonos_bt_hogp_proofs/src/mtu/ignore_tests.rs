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

//! Answers to the exchange that are never trusted: under 23, over the
//! offer, malformed, or an error. The default 23 holds and discovery goes
//! on. A server that then answers in longer pieces than 23 is still read
//! whole, since every read goes on while answers come full.

use super::take_tests::{long_map, run_at};
use crate::sim::report_devices::{ready, report_keyboard, MODE};
use crate::sim::{Peripheral, Run};
use nonos_bt_hogp::host::{Outcome, TIMEOUT_MS};
use nonos_bt_hogp::HogpError;
use nonos_bt_wire::att::error::{INSUFFICIENT_AUTHENTICATION, REQUEST_NOT_SUPPORTED};

fn with(f: impl FnOnce(&mut Peripheral)) -> Run {
    let mut peer = Peripheral::new(report_keyboard(&long_map()));
    f(&mut peer);
    let mut run = Run::with(peer);
    ready(&mut run);
    assert_eq!(run.peer.writes[0], (MODE, vec![1], true));
    run
}

#[test]
fn an_answer_under_23_or_malformed_is_ignored() {
    for raw in [vec![22, 0], vec![0, 0], vec![23], vec![100, 0, 0], vec![]] {
        assert_eq!(with(|p| p.mtu_reply = Some(raw.clone())).host.mtu(), 23, "{raw:?}");
    }
}

#[test]
fn an_answer_over_the_offer_is_ignored_and_the_longer_answers_still_read() {
    // The server takes 247 as the bearer's MTU from its side, and sends
    // pieces of 246; the host keeps 23 and reads on while they come full.
    let (run, offsets) = run_at(248);
    assert_eq!(run.host.mtu(), 23);
    // 54 octets is a full answer at 23, so one more Read Blob finds the end.
    assert_eq!(offsets, [246, 300]);
    assert_eq!(run.peer.writes[0], (MODE, vec![1], true));
}

#[test]
fn an_error_keeps_the_default_and_asks_for_nothing() {
    for code in [REQUEST_NOT_SUPPORTED, INSUFFICIENT_AUTHENTICATION] {
        let run = with(|p| p.refuse_op = Some((0x02, code)));
        assert_eq!(run.host.mtu(), 23);
    }
}

#[test]
fn no_answer_ends_the_bearer_at_the_timeout() {
    let mut peer = Peripheral::new(report_keyboard(&long_map()));
    peer.mute = Some(0x02);
    let mut run = Run::with(peer);
    assert_eq!(run.go(), None);
    let timeout = Some(Outcome::Failed(HogpError::Timeout));
    assert_eq!(run.host.poll(TIMEOUT_MS).outcome, timeout);
}
