// NONOS Operating System (AGPL-3.0-or-later)
/* A caller that ended without resetting its conversation: its slot, its
 * tunnel and the reply kept for it are freed, and a living caller's stay. */

use std::cell::Cell;

use crate::clients::{Clients, MAX_CLIENTS};
use crate::kept::{answer, forget_ended};
use crate::who::{pid_of, who};

/*
 * Only a caller resets its own conversation, so each one that crashed kept a
 * slot for good, and after MAX_CLIENTS of them the Nym route refused every
 * program until a reboot.
 */
#[test]
fn ended_callers_give_their_slots_back_and_living_ones_keep_theirs() {
    let mut c = Clients::new();
    for pid in 0..MAX_CLIENTS as u32 {
        assert!(c.get(who(100 + pid, 0)).is_some());
    }
    assert!(c.get(who(999, 0)).is_none(), "the table is full");
    let gone = c.ended(|pid| pid % 2 == 0);
    assert_eq!(gone.len(), MAX_CLIENTS / 2);
    assert!(gone.iter().all(|&w| pid_of(w) % 2 == 1));
    for w in gone {
        c.drop_client(w);
    }
    assert!(c.get(who(999, 0)).is_some(), "a new caller is served");
    assert!(c.ended(|_| true).is_empty(), "nobody ended, nothing to free");
}

fn carried_again(pid: u32) -> bool {
    let called = Cell::new(false);
    let _ = answer(who(pid, 0), 1, b"x", |_, _| {
        called.set(true);
        vec![1]
    });
    called.get()
}

#[test]
fn the_reply_kept_for_an_ended_caller_is_forgotten() {
    let (ended, living) = (70_001u32, 70_002u32);
    for pid in [ended, living] {
        assert!(carried_again(pid), "the first ask is carried");
        assert!(!carried_again(pid), "the same ask again is answered from what was kept");
    }
    forget_ended(|pid| pid != ended);
    assert!(carried_again(ended), "nothing kept for the caller that ended");
    assert!(!carried_again(living), "the living caller's reply is still kept");
}
