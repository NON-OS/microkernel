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

//! Buy's focus guard: armed only on the drawn consent with focus held for
//! ARM_MS, which starts again whenever focus is lost and won back.

use crate::order::terms::arming::{armed, buy_armed, look, Guard, Look, ARM_MS};

const T: u64 = 50_000;

fn held(g: Guard, focused: bool, now: u64) -> Guard {
    match look(g, focused, now) {
        Look::Held(g) => g,
        Look::Lost => panic!("lost at {now}"),
    }
}

#[test]
fn buy_is_refused_before_one_second_of_focus() {
    assert_eq!(ARM_MS, 1_000);
    assert!(!armed(true, true, T, T));
    assert!(!armed(true, true, T, T + ARM_MS - 1));
    assert!(armed(true, true, T, T + ARM_MS));
    assert!(!armed(true, true, T, T - 1), "a time before focus is no time held");
    let g = held(Guard { drawn: true, since: None }, true, T);
    assert_eq!(g.since, Some(T));
    assert!(!buy_armed(g, true, T + ARM_MS - 1));
    assert!(buy_armed(g, true, T + ARM_MS));
}

#[test]
fn buy_is_refused_without_focus_or_before_the_consent_is_drawn() {
    assert!(!armed(true, false, T, T + 10 * ARM_MS));
    assert!(!armed(false, true, T, T + 10 * ARM_MS));
    let undrawn = held(Guard { drawn: false, since: Some(T) }, true, T + 5 * ARM_MS);
    assert_eq!(undrawn, Guard::default(), "focus before the consent is drawn counts for nothing");
    assert!(!buy_armed(undrawn, true, T + 10 * ARM_MS));
    let never = held(Guard { drawn: true, since: None }, false, T);
    assert!(!buy_armed(never, false, T + 10 * ARM_MS));
}

#[test]
fn buy_is_armed_again_only_after_a_fresh_second() {
    let g = held(Guard { drawn: true, since: None }, true, T);
    assert!(buy_armed(g, true, T + 3 * ARM_MS));
    assert_eq!(look(g, false, T + 3 * ARM_MS), Look::Lost);
    assert!(!buy_armed(g, false, T + 3 * ARM_MS), "not with focus elsewhere");
    let back = T + 4 * ARM_MS;
    let regained = held(Guard { drawn: true, since: None }, true, back);
    assert!(!buy_armed(regained, true, back + 1), "the old second does not carry over");
    assert!(!buy_armed(regained, true, back + ARM_MS - 1));
    assert!(buy_armed(regained, true, back + ARM_MS));
}

#[test]
fn focus_lost_over_the_drawn_consent_is_lost_only_once_it_was_held() {
    let drawn = Guard { drawn: true, since: None };
    assert_eq!(look(drawn, false, T), Look::Held(drawn), "never held: nothing lost");
    let g = held(drawn, true, T);
    assert_eq!(look(g, true, T + 1), Look::Held(g), "held: the second keeps running");
    assert_eq!(look(g, false, T + 1), Look::Lost);
}
