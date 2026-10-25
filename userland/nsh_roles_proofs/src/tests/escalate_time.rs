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

//! An escalation's time: within its own grant's window, while the base
//! session still runs, and ending at most one hour from now.

use crate::build::grant::GrantB;
use crate::build::run::Setup;
use crate::build::{HOUR, NOW};
use nsh_roles::{Refusal, MAX_ESCALATION_MS};
use nsh_wire::Reason;

fn at(now: u64, g: GrantB) -> Result<(), Refusal> {
    Setup { now, ..Setup::default() }.escalate(&Setup::base(), &g).map(|_| ())
}

#[test]
fn an_escalation_holds_from_not_before_until_just_before_not_after() {
    let g = GrantB::escalation;
    let (nb, na) = (g().not_before_ms, g().not_after_ms);
    assert_eq!(at(nb - 1, g()), Err(Refusal::NotYet));
    assert_eq!(at(nb, g()), Ok(()));
    assert_eq!(at(na - 1, g()), Ok(()));
    assert_eq!(at(na, g()), Err(Refusal::Expired));
}

#[test]
fn an_escalation_is_refused_once_the_base_session_has_ended() {
    let ends = Setup::base().ends_ms;
    let g = || GrantB {
        not_before_ms: ends - 10,
        not_after_ms: ends + HOUR / 2,
        ..GrantB::escalation()
    };
    assert_eq!(at(ends - 1, g()), Ok(()));
    assert_eq!(at(ends, g()), Err(Refusal::BaseEnded));
    assert_eq!(Refusal::BaseEnded.reason(), Reason::GrantTime);
}

#[test]
fn an_escalation_window_is_at_most_one_hour() {
    assert_eq!(MAX_ESCALATION_MS, 3_600_000);
    let window = |nb, na| GrantB { not_before_ms: nb, not_after_ms: na, ..GrantB::escalation() };
    assert_eq!(at(NOW, window(NOW, NOW + MAX_ESCALATION_MS)), Ok(()));
    assert_eq!(at(NOW, window(NOW, NOW + MAX_ESCALATION_MS + 1)), Err(Refusal::TooLong));
    assert_eq!(Refusal::TooLong.reason(), Reason::EscTooLong);
}

#[test]
fn a_long_window_is_refused_even_when_it_is_nearly_over() {
    let g = GrantB { not_before_ms: NOW - HOUR, not_after_ms: NOW + 1, ..GrantB::escalation() };
    assert_eq!(at(NOW, g.clone()), Err(Refusal::TooLong));
    assert_eq!(at(NOW + 5 * HOUR, g), Err(Refusal::TooLong), "before the time is judged");
}
