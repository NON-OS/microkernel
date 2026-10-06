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

//! No tick waits on proxies for long, and a proxy that is busy is left be.

use crate::browser::net::mixnet::fault::{fault, Fault};
use crate::browser::net::mixnet::pace::{Pace, POLL_MS, REASK_GAP_MS, TICK_BUDGET_MS};

const NYM: u32 = 41;
const ANON: u32 = 42;

#[test]
fn a_tick_stops_calling_once_its_budget_is_spent() {
    let mut pace = Pace::new();
    pace.new_tick(0);
    let mut now = 0;
    let mut calls = 0;
    while pace.may_ask(NYM, now) {
        pace.asked(NYM, now, now + 40, true);
        now += 40;
        calls += 1;
    }
    assert_eq!(calls, 2, "40 ms, then 80: the second call crossed the budget");
    assert!(now <= TICK_BUDGET_MS + POLL_MS as i64, "a tick waits at most its budget and one call");
    pace.new_tick(now);
    assert!(pace.may_ask(NYM, now), "the next tick has its budget again");
}

#[test]
fn a_proxy_that_left_a_call_unanswered_is_left_alone_for_a_while() {
    let mut pace = Pace::new();
    pace.new_tick(0);
    assert!(pace.may_ask(NYM, 0));
    pace.asked(NYM, 0, 60, false);
    for tick in 1..4 {
        let now = 60 + tick * 30;
        pace.new_tick(now);
        assert!(!pace.may_ask(NYM, now), "a busy proxy is not asked every tick");
        assert!(pace.may_ask(ANON, now), "another proxy still is");
    }
    pace.new_tick(60 + REASK_GAP_MS);
    assert!(pace.may_ask(NYM, 60 + REASK_GAP_MS));
    pace.asked(NYM, 260, 261, true);
    pace.new_tick(262);
    assert!(pace.may_ask(NYM, 262), "an answer ends the rest at once");
}

#[test]
fn a_budget_nobody_renewed_is_renewed_in_time() {
    let mut pace = Pace::new();
    pace.new_tick(0);
    pace.asked(NYM, 0, 100, true);
    assert!(!pace.may_ask(NYM, 100));
    assert!(pace.may_ask(NYM, 1_100), "slowed, never stopped");
}

#[test]
fn not_now_is_told_from_not_there() {
    assert_eq!(fault(-110), Fault::Unanswered, "timed out");
    assert_eq!(fault(-16), Fault::Unanswered, "eight of our calls already wait there");
    assert_eq!(fault(-11), Fault::Unanswered, "its queue is full");
    for rc in [-2, -3, -13, -12, -22, -1] {
        assert_eq!(fault(rc), Fault::Refused, "rc {rc}: asking again changes nothing");
    }
}
