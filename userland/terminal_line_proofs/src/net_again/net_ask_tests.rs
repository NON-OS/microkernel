// NONOS Operating System (AGPL-3.0-or-later)
//! A plain `linux` run that ends with LINUX_ASK_NET is started once more as
//! `tool.linux.net` with the same argv and streams; any other end, and any
//! second ask, starts nothing. What it was handed before it was heard from
//! is handed again, in order.

use super::net_ask::{ended, NetAsk, NET_TOOL, PLAIN_TOOL};
use super::stdin_kept::{Kept, KEPT_MAX};

const ASK: i32 = super::foreign_frame::LINUX_ASK_NET;

fn held() -> Option<NetAsk> {
    NetAsk::of(PLAIN_TOOL, 7, b"linux\0profanity", 3)
}

#[test]
fn only_a_plain_linux_run_may_ask() {
    assert!(held().is_some());
    for service in [NET_TOOL, &b"tool.grex"[..], b"tool.linux.netx", b""] {
        assert_eq!(NetAsk::of(service, 7, b"linux", 3), None, "{service:?}");
    }
}

#[test]
fn ask_net_starts_it_once_with_its_own_argv_and_streams() {
    let mut slot = held();
    let again = ended(&mut slot, 7, ASK, ASK).expect("started again");
    assert_eq!((again.argv.as_slice(), again.streams), (&b"linux\0profanity"[..], 3));
    assert_eq!(slot, None);
    assert_eq!(ended(&mut slot, 7, ASK, ASK), None, "never a second time");
}

#[test]
fn any_other_end_or_another_pid_starts_nothing() {
    for status in [0, 1, 2, 37, 75, 137, 255, -1, ASK + 1, ASK & 0xff] {
        let mut slot = held();
        assert_eq!(ended(&mut slot, 7, status, ASK), None, "{status}");
        assert_eq!(slot, None, "its chance is spent when it ends: {status}");
    }
    let mut slot = held();
    assert_eq!(ended(&mut slot, 8, ASK, ASK), None);
    assert_eq!(slot, held(), "another job's end leaves it");
}

#[test]
fn ask_net_is_no_status_a_guest_can_give() {
    assert!(!(0..=255).contains(&ASK));
}

#[test]
fn what_was_handed_before_it_was_heard_from_is_handed_again_in_order() {
    let mut kept = Kept::new();
    for msg in [&b"hello\n"[..], b"x", b"\x04"] {
        kept.handed(msg);
    }
    assert_eq!(kept.take(), [b"hello\n".to_vec(), b"x".to_vec(), b"\x04".to_vec()]);
    assert!(kept.take().is_empty(), "taken once");
}

#[test]
fn once_the_program_writes_or_past_the_cap_nothing_is_kept() {
    let mut kept = Kept::new();
    kept.handed(b"a");
    kept.heard();
    kept.handed(b"b");
    assert!(kept.take().is_empty());
    let mut kept = Kept::new();
    kept.handed(&vec![b'a'; KEPT_MAX]);
    kept.handed(b"one more");
    kept.handed(b"after");
    assert!(kept.take().is_empty(), "a cut keep would replay half");
}
