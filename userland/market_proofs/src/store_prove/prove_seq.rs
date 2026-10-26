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

//! The sequence a yes starts, call by call, as the note orders it: op 8,
//! MkProveJob, app.prove.job looked up, PROVE, STATUS, GETs to the total.

use alloc::vec::Vec;

use super::proof_event::{Call, Event};
use super::proof_fail::Fail;
use super::proof_seq::{ended, Phase, DONE, READING};
use super::proof_step::step;

/// Each event in turn from `from`: where it ends, and every call on the way.
pub fn walk(from: Phase, events: &[Event]) -> (Phase, Vec<Call>) {
    events.iter().fold((from, Vec::new()), |(p, mut calls), e| {
        let (next, call) = step(p, *e);
        calls.push(call);
        (next, calls)
    })
}

#[test]
fn a_yes_asks_the_box_then_the_prover_then_follows_it_to_the_last_byte() {
    use Event::*;
    let events = [Tick, Policy(Ok(())), Job(0), Tick, Up, Proved(Ok(())), Tick];
    let more = [Status(Ok((READING, 0))), Tick, Status(Ok((DONE, 70_000))), Got(Ok(65_536))];
    let (end, calls) = walk(Phase::Policy, &[&events[..], &more, &[Got(Ok(4_464))]].concat());
    let want = [Call::Policy, Call::AskJob, Call::Lookup, Call::Lookup, Call::Prove, Call::None];
    let after = [Call::Status, Call::None, Call::Status, Call::Get(0), Call::Get(65_536)];
    assert_eq!(calls, [&want[..], &after, &[Call::None]].concat());
    assert_eq!(end, Phase::Held);
    assert!(ended(end));
}

#[test]
fn nothing_is_asked_of_the_prover_before_the_box_names_this_listing() {
    for f in [Fail::NoBox, Fail::NotNamed] {
        let (end, calls) = walk(Phase::Policy, &[Event::Tick, Event::Policy(Err(f)), Event::Tick]);
        assert_eq!(end, Phase::Failed(f, false));
        assert_eq!(calls, [Call::Policy, Call::None, Call::None]);
    }
    let (end, calls) = walk(Phase::Policy, &[Event::Job(0), Event::Up]);
    assert_eq!((end, calls), (Phase::Policy, alloc::vec![Call::None, Call::None]), "no box yet");
}
