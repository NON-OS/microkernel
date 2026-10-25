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

//! The answer over every bit, both broker states, and senders live, dead,
//! started before, at and after their message: exactly two bits askable,
//! never a mask, never to a non-broker, never yes for a reused pid.

use super::rule::{answer, Target, ASKABLE, EINVAL, EPERM, ESRCH};

const WINDOW: u64 = 1 << 24;
const INSTALL: u64 = 1 << 30;
const ADMIN: u64 = 1 << 9;

fn live(caps: u64, started_ms: u64) -> Option<Target> {
    Some(Target { live: true, started_ms, caps })
}

#[test]
fn exactly_two_bits_are_askable_and_never_a_mask() {
    assert_eq!(ASKABLE, [WINDOW, INSTALL]);
    for bit in 0..64 {
        let b = 1u64 << bit;
        let r = answer(true, b, live(u64::MAX, 0), 10);
        assert_eq!(r, if b == WINDOW || b == INSTALL { 1 } else { EINVAL }, "bit {bit}");
    }
    for mask in [0, WINDOW | INSTALL, u64::MAX, WINDOW | 1] {
        assert_eq!(answer(true, mask, live(u64::MAX, 0), 10), EINVAL, "{mask:#x}");
    }
}

#[test]
fn a_non_broker_is_refused_whatever_it_asks() {
    for bit in [WINDOW, INSTALL, 0, u64::MAX] {
        assert_eq!(answer(false, bit, live(u64::MAX, 0), 10), EPERM);
        assert_eq!(answer(false, bit, None, 10), EPERM);
    }
}

#[test]
fn it_answers_what_the_live_sender_holds_admin_standing_in_for_a_launcher() {
    assert_eq!(answer(true, WINDOW, live(WINDOW, 5), 10), 1);
    assert_eq!(answer(true, WINDOW, live(ADMIN, 5), 10), 1, "as can_spawn_window");
    assert_eq!(answer(true, WINDOW, live(INSTALL, 5), 10), 0);
    assert_eq!(answer(true, INSTALL, live(INSTALL, 5), 10), 1);
    assert_eq!(answer(true, INSTALL, live(ADMIN | WINDOW, 5), 10), 0, "Admin is no AppInstall");
}

#[test]
fn a_dead_or_missing_or_later_process_is_no_sender() {
    let dead = Some(Target { live: false, started_ms: 0, caps: u64::MAX });
    assert_eq!(answer(true, WINDOW, dead, 10), ESRCH);
    assert_eq!(answer(true, WINDOW, None, 10), ESRCH);
    // The pid reused by a process started after the message: never yes.
    assert_eq!(answer(true, WINDOW, live(u64::MAX, 11), 10), ESRCH);
    assert_eq!(answer(true, INSTALL, live(u64::MAX, u64::MAX), 0), ESRCH);
    assert_eq!(answer(true, WINDOW, live(WINDOW, 10), 10), 1, "started as it sent");
}
