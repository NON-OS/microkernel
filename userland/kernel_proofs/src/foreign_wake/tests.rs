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

use super::rule::{look, Look::*};

#[test]
fn a_message_to_the_callers_own_inbox_ends_the_wait() {
    assert_eq!(look(false, 0, 0), Sleep, "nothing came");
    assert_eq!(look(false, 1, 0), Inbox, "the first message wakes it");
    assert_eq!(look(false, 9, 4), Inbox, "and so does any after it was told");
}

#[test]
fn a_message_left_unread_wakes_it_once() {
    let mut heard = 0;
    let mut woken = 0;
    for _ in 0..5 {
        if look(false, 3, heard) == Inbox {
            heard = 3;
            woken += 1;
        }
    }
    assert_eq!(woken, 1, "a wait woken by a count it was told of spins");
}

#[test]
fn a_trapped_guest_goes_first_and_the_inbox_is_told_next() {
    assert_eq!(look(true, 0, 0), Frame);
    assert_eq!(look(true, 2, 0), Frame, "a frame is not held back by the inbox");
    assert_eq!(look(false, 2, 0), Inbox, "the message is told on the next look");
}

#[test]
fn a_reused_pid_is_told_of_its_new_inbox() {
    assert_eq!(look(false, 1, 7), Inbox, "a count below the one kept is new");
}
