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

//! A sleep or a stop never lands over a kill (L77d, L77e): each guard is
//! read whole, whitespace aside, and sits before its one store under its one
//! lock, so neither a guard moved below the store nor one asked and then
//! ignored passes.

const SLEEP: &str = include_str!("../../../../src/process/scheduler/dispatch/sleep_enter.rs");
const STOP: &str = include_str!("../../../../src/process/signal/delivery/default.rs");

#[test]
fn a_sleep_or_a_stop_never_lands_over_a_kill() {
    let (flat, dead) = (|s: &str| s.split_whitespace().collect::<String>(), DEAD);
    let s = flat(&SLEEP[SLEEP.find("fn enter_sleep(").expect("enter_sleep")..]);
    let guard =
        s.find(&format!("|s|!matches!(**s,{dead}));if!live{{return;}}remove_from_run_queue"));
    assert!(guard.expect("the sleep's guard") < s.find("=ProcessState::Sleeping").unwrap());
    let t = flat(&STOP[STOP.find("DefaultAction::Stop =>").expect("the stop")..]);
    let t = t.split_once("DefaultAction::Continue").expect("the continue").0;
    let stop = format!("lock();if!matches!(*state,{dead}){{*state=ProcessState::Stopped;}}}});}}");
    assert!(t.ends_with(&stop) && [&*s, t].iter().all(|x| x.matches("state.lock()").count() == 1));
}

const DEAD: &str = "ProcessState::Zombie(_)|ProcessState::Terminated(_)";
