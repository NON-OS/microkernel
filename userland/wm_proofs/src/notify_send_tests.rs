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

//! A lifecycle notification offered to a subscriber whose inbox is full, gone
//! or refusing (protocol/notify_send.rs).

use crate::protocol::notify_send::{offer, BUSY_TRIES, EBUSY};

const ENOENT: i64 = -2;
const EPERM: i64 = -1;

/// Offer to a subscriber that answers `answers` in turn (then 0); returns
/// whether it was dropped, how many sends and how many yields it took.
fn run(answers: &[i64]) -> (bool, usize, usize) {
    let (mut sends, mut waits) = (0usize, 0usize);
    let dropped = offer(
        || {
            sends += 1;
            answers.get(sends - 1).copied().unwrap_or(0)
        },
        || waits += 1,
    );
    (dropped, sends, waits)
}

/// The desktop shell's inbox, full of mirrored pointer motion, as a window
/// opens: the event waits for it, and the shell stays subscribed. Before, the
/// first EBUSY dropped the shell for good and the dock stopped marking apps.
#[test]
fn a_full_inbox_is_waited_on_and_the_subscriber_kept() {
    assert_eq!(run(&[EBUSY, EBUSY, 0]), (false, 3, 2));
}

#[test]
fn an_inbox_full_for_long_loses_the_event_not_the_subscriber() {
    let always = [EBUSY; 64];
    let (dropped, sends, waits) = run(&always);
    assert!(!dropped);
    assert_eq!(sends, BUSY_TRIES as usize);
    assert_eq!(waits, BUSY_TRIES as usize - 1, "no yield after the last try");
}

#[test]
fn a_gone_or_refusing_inbox_drops_the_subscriber_at_once() {
    assert_eq!(run(&[ENOENT]), (true, 1, 0));
    assert_eq!(run(&[EPERM]), (true, 1, 0));
    assert_eq!(run(&[EBUSY, ENOENT]), (true, 2, 1));
}

#[test]
fn a_delivered_event_is_sent_once() {
    assert_eq!(run(&[0]), (false, 1, 0));
}
