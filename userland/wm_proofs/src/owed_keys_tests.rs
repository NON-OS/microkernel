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

//! A window is told it has the keyboard even when its owner's inbox is full
//! as focus comes to it (focus/owed_keys.rs). A Wayland guest does not read
//! its inbox while its client starts (no seat bound yet, or the client
//! behind on what it was sent), and the input router fills it with pointer
//! frames meanwhile. The one KEYS notice used to be offered a few times and
//! dropped, and the window never took a key.

use crate::focus::model::FocusedRef;
use crate::focus::notice::keys_notices;
use crate::focus::owed_keys::{Owed, OwedKeys, OWED_MAX};
use crate::protocol::notify_send::{offer, BUSY_TRIES, EBUSY};

const GUEST: u32 = 40;
const WINDOW: u32 = 0x4C4E_0001;

/// An owner's bounded inbox; it reads nothing until `reading`.
struct Inbox {
    room: usize,
    got: Vec<Owed>,
}

impl Inbox {
    fn full() -> Inbox {
        Inbox { room: 0, got: Vec::new() }
    }

    /// Whether the send found the inbox full, as a send's EBUSY.
    fn send(&mut self, n: &Owed) -> bool {
        if self.room == 0 {
            return true;
        }
        self.room -= 1;
        self.got.push(*n);
        false
    }
}

fn focus_comes(was: Option<FocusedRef>) -> [Option<(u32, u32, bool)>; 2] {
    keys_notices(was, Some(FocusedRef { owner_pid: GUEST, window_id: WINDOW }))
}

#[test]
fn a_notice_offered_once_to_a_full_inbox_was_lost() {
    // The window manager's previous path: offer, give up, never again.
    let mut inbox = Inbox::full();
    for (owner_pid, window_id, has) in focus_comes(None).into_iter().flatten() {
        let n = Owed { owner_pid, window_id, has };
        let gone = offer(|| if inbox.send(&n) { EBUSY } else { 0 }, || {});
        assert!(!gone, "a full inbox is not a gone one");
    }
    inbox.room = 8;
    assert!(
        inbox.got.is_empty(),
        "nothing is sent again once the guest reads: the keyboard is lost"
    );
}

#[test]
fn a_notice_a_full_inbox_refused_lands_once_the_owner_reads() {
    let (mut owed, mut inbox) = (OwedKeys::new(), Inbox::full());
    for (owner_pid, window_id, has) in focus_comes(None).into_iter().flatten() {
        let n = Owed { owner_pid, window_id, has };
        owed.tell(n, BUSY_TRIES, |n| inbox.send(n), || {});
    }
    assert!(inbox.got.is_empty() && !owed.is_empty(), "owed while the inbox is full");
    // Turns pass with the guest still not reading: it stays owed.
    for _ in 0..50 {
        owed.offer_again(|n| inbox.send(n));
    }
    assert!(!owed.is_empty());
    inbox.room = 8;
    owed.offer_again(|n| inbox.send(n));
    assert_eq!(inbox.got, vec![Owed { owner_pid: GUEST, window_id: WINDOW, has: true }]);
    assert!(owed.is_empty(), "delivered once, owed no more");
}

#[test]
fn only_the_latest_word_on_a_windows_keyboard_is_owed() {
    let (mut owed, mut inbox) = (OwedKeys::new(), Inbox::full());
    let gained = Owed { owner_pid: GUEST, window_id: WINDOW, has: true };
    let lost = Owed { has: false, ..gained };
    owed.tell(gained, BUSY_TRIES, |n| inbox.send(n), || {});
    owed.tell(lost, BUSY_TRIES, |n| inbox.send(n), || {});
    inbox.room = 8;
    owed.offer_again(|n| inbox.send(n));
    assert_eq!(inbox.got, vec![lost], "the window is told it lost the keyboard, once");
}

#[test]
fn a_notice_delivered_now_settles_one_owed_before() {
    let (mut owed, mut inbox) = (OwedKeys::new(), Inbox::full());
    let gained = Owed { owner_pid: GUEST, window_id: WINDOW, has: true };
    owed.tell(gained, BUSY_TRIES, |n| inbox.send(n), || {});
    inbox.room = 1;
    owed.tell(gained, BUSY_TRIES, |n| inbox.send(n), || {});
    assert!(owed.is_empty());
    owed.offer_again(|n| inbox.send(n));
    assert_eq!(inbox.got.len(), 1, "not told twice");
}

#[test]
fn an_owner_gone_or_refusing_is_owed_nothing() {
    let mut owed = OwedKeys::new();
    let n = Owed { owner_pid: GUEST, window_id: WINDOW, has: true };
    owed.owe(n);
    // the project exited (resend_owed answers false for a dead pid).
    owed.offer_again(|_| false);
    assert!(owed.is_empty());
}

#[test]
fn a_full_table_gives_way_oldest_first() {
    let mut owed = OwedKeys::new();
    for w in 0..OWED_MAX as u32 + 1 {
        owed.owe(Owed { owner_pid: GUEST, window_id: w, has: true });
    }
    let mut seen = Vec::new();
    owed.offer_again(|n| {
        seen.push(n.window_id);
        false
    });
    assert_eq!(seen.len(), OWED_MAX);
    assert!(!seen.contains(&0) && seen.contains(&(OWED_MAX as u32)));
}

#[test]
fn the_tries_before_owing_yield_between_them() {
    let (mut owed, mut inbox, mut waits) = (OwedKeys::new(), Inbox::full(), 0);
    let n = Owed { owner_pid: GUEST, window_id: WINDOW, has: true };
    owed.tell(n, BUSY_TRIES, |n| inbox.send(n), || waits += 1);
    assert_eq!(waits, BUSY_TRIES as usize - 1);
}
