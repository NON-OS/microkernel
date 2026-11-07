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

//! A connection that comes while a session runs is told BUSY and held until
//! its peer goes or HOLD_MS passes, never more than HELD_MAX at once, and
//! the listener is asked at most once every ASK_EVERY_MS.

use crate::read::Read;
use crate::turn_away::{Away, ASK_EVERY_MS, HELD_MAX, HOLD_MS, LET_GO_IDLE_MS, LET_GO_PASSES};

const SESSION: &str = include_str!("../../../capsule_nsh_front/src/session.rs");
const NEWCOMERS: &str = include_str!("../../../capsule_nsh_front/src/newcomers.rs");

fn at(src: &str, s: &str) -> usize {
    src.find(s).unwrap_or_else(|| panic!("{s} is gone"))
}

#[test]
fn the_listener_is_asked_at_once_then_once_a_period() {
    let mut a = Away::new();
    assert!(a.due(1_000), "the first round");
    assert!(!a.due(1_000));
    assert!(!a.due(1_000 + ASK_EVERY_MS - 1));
    assert!(a.due(1_000 + ASK_EVERY_MS));
    assert!(!a.due(1_000 + ASK_EVERY_MS), "counted from the round that ran");
}

#[test]
fn a_clock_that_steps_back_waits_rather_than_spins() {
    let mut a = Away::new();
    assert!(a.due(5_000));
    assert!(!a.due(10), "an earlier time is no period later");
}

#[test]
fn each_round_numbers_its_reads_afresh_and_never_zero() {
    let mut a = Away::new();
    assert_eq!(a.seq(), 1, "before any round");
    assert!(a.due(0));
    let first = a.seq();
    assert!(a.due(ASK_EVERY_MS));
    assert_ne!(a.seq(), first);
    let mut last = a.seq();
    for _ in 0..10 {
        a.next_round();
        assert_ne!(a.seq(), last);
        assert_ne!(a.seq(), 0);
        last = a.seq();
    }
}

#[test]
fn no_more_than_held_max_are_held() {
    let mut a = Away::new();
    for h in 0..HELD_MAX as u32 {
        assert_eq!(a.hold(10 + h, 0), None);
    }
    assert_eq!(a.hold(99, 0), Some(99), "one more is closed at once");
    assert!(a.held().iter().all(Option::is_some));
    assert!(!a.held().contains(&Some(99)));
}

#[test]
fn a_held_newcomer_stays_while_it_is_quiet_or_sends() {
    let mut a = Away::new();
    a.hold(7, 100);
    assert!(!a.settle(7, Read::Idle, 100));
    assert!(!a.settle(7, Read::Data, 100 + HOLD_MS - 1), "its hello is read and dropped");
    assert!(!a.settle(7, Read::Idle, 100 + HOLD_MS - 1));
    assert!(a.holding());
}

#[test]
fn it_goes_when_its_peer_goes_or_its_hold_runs_out() {
    let mut a = Away::new();
    a.hold(7, 100);
    a.hold(8, 100);
    assert!(a.settle(7, Read::Gone, 101));
    assert!(a.settle(8, Read::Idle, 100 + HOLD_MS));
    assert!(!a.holding());
    assert_eq!(a.hold(9, 200), None, "a freed slot is taken again");
}

#[test]
fn a_hold_runs_out_even_for_a_peer_that_keeps_sending() {
    let mut a = Away::new();
    a.hold(7, 0);
    assert!(a.settle(7, Read::Data, HOLD_MS), "no peer keeps a slot past HOLD_MS");
}

#[test]
fn a_handle_not_held_is_not_closed_twice() {
    let mut a = Away::new();
    assert!(!a.settle(3, Read::Gone, 0));
    a.hold(3, 0);
    assert!(a.settle(3, Read::Gone, 0));
    assert!(!a.settle(3, Read::Gone, 0), "it left its slot the first time");
}

#[test]
fn a_lost_listener_is_never_asked_again_and_is_reported() {
    let mut a = Away::new();
    assert!(a.listening());
    a.hold(4, 0);
    a.lose_listener();
    assert!(!a.listening());
    for t in [0, ASK_EVERY_MS, 10 * ASK_EVERY_MS] {
        assert!(!a.due(t));
    }
    assert!(a.settle(4, Read::Gone, 1), "the ones held are still let go");
}

#[test]
fn every_session_turn_asks_for_newcomers() {
    let run = at(SESSION, "fn run(");
    let looped = run + at(&SESSION[run..], "while !s.is_over()");
    let round = run + at(&SESSION[run..], "newcomers::round(net, listener, away);");
    let feed = run + at(&SESSION[run..], "net.recv(child, seq)");
    assert!(looped < round && round < feed, "in the loop, each turn");
}

#[test]
fn a_session_that_cannot_start_says_why_and_every_newcomer_is_let_go() {
    let serve = at(SESSION, "pub fn serve(");
    let body = &SESSION[serve..at(SESSION, "fn start(")];
    let refused = at(body, "Err(why) => {");
    assert!(at(body, "let _ = mk_debug(text.as_ptr(), text.len());") > refused, "said on serial");
    let told = at(body, "turned_away(why.reason()).unwrap_or_else(busy);");
    assert!(told > refused);
    assert!(
        at(body, "newcomers::tell(net, &mut away, child, newcomers::uptime(), &answer);") > told
    );
    assert!(!body[refused..].contains("net.close(child)"), "never closed without a word");
    let after = at(body, "newcomers::let_go(net, &mut away);");
    assert!(after > refused && after < at(body, "away.listening()"));
}

#[test]
fn busy_is_sent_before_a_newcomer_is_held_or_closed() {
    let tell = at(NEWCOMERS, "pub fn tell(");
    let body = &NEWCOMERS[tell..];
    let sent = at(body, "net.send(child, answer)");
    assert!(sent < at(body, "away.hold(child, now)"));
    assert!(NEWCOMERS.contains("Accept::Child(child) => tell(net, away, child, now, &busy()),"));
    assert!(NEWCOMERS.contains("use nsh_server::busy;"), "nsh/1's own plaintext BUSY frame");
    assert!(NEWCOMERS.contains("Accept::Gone => away.lose_listener(),"));
}

#[test]
fn what_is_left_is_released_once() {
    let mut a = Away::new();
    a.hold(5, 0);
    a.hold(6, 0);
    let mut left: Vec<u32> = a.release_all().into_iter().flatten().collect();
    left.sort_unstable();
    assert_eq!(left, [5, 6]);
    assert!(!a.holding());
    assert!(a.release_all().iter().all(Option::is_none), "nothing is closed twice");
}

#[test]
fn the_wait_after_a_session_is_bounded_without_the_clock() {
    assert_eq!(LET_GO_PASSES * LET_GO_IDLE_MS, HOLD_MS);
    let let_go = at(NEWCOMERS, "pub fn let_go(");
    let body = &NEWCOMERS[let_go..];
    let looped = at(body, "while away.holding() && passes < LET_GO_PASSES {");
    assert!(looped < at(body, "passes += 1;"));
    assert!(looped < at(body, "away.release_all()"), "whatever is left is closed after");
}

#[test]
fn holds_are_timed_on_the_monotonic_uptime() {
    assert!(NEWCOMERS.contains("u64::try_from(mk_uptime_ms()).unwrap_or(0)"));
    assert!(!NEWCOMERS.contains("clock::now()"), "never the wall clock, which can be unset");
}
