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

//! STATUS, one at a time (ruling 3): one that times out is given up and the
//! next tick asks again, since the kernel drops a reply whose call gave up;
//! the job's quiet spell ends it. And the GETs: in order, to exactly the
//! total, nothing past it.

use super::proof_event::{Call, Event};
use super::proof_fail::Fail;
use super::proof_get::GET_MAX;
use super::proof_seq::{Phase, DEVICE, DONE, ISOLATION};
use super::prove_seq::walk;

#[test]
fn a_timed_out_status_is_given_up_and_asked_again() {
    let (held, calls) = walk(Phase::Proving { stage: DEVICE, out: false }, &[Event::Tick]);
    assert_eq!(
        (held, &calls[..]),
        (Phase::Proving { stage: DEVICE, out: true }, &[Call::Status][..])
    );
    let (late, calls) = walk(held, &[Event::StatusLate]);
    assert_eq!((late, &calls[..]), (Phase::Proving { stage: DEVICE, out: false }, &[Call::None][..]));
    let (again, calls) = walk(late, &[Event::Tick, Event::Tick]);
    assert_eq!(again, held, "one STATUS outstanding again");
    assert_eq!(calls, [Call::Status, Call::None], "never two at once");
    let (told, calls) = walk(again, &[Event::Status(Ok((ISOLATION, 0))), Event::Tick]);
    assert_eq!(
        (told, &calls[..]),
        (Phase::Proving { stage: ISOLATION, out: true }, &[Call::None, Call::Status][..])
    );
    assert_eq!(walk(late, &[Event::Late]).0, Phase::Failed(Fail::Quiet, true));
}

#[test]
fn a_container_comes_in_order_to_exactly_its_total() {
    let at = |at, total| Phase::Fetching { at, total };
    let max = GET_MAX as u32;
    assert_eq!(
        walk(at(0, max + 1), &[Event::Got(Ok(GET_MAX))]),
        (at(max, max + 1), alloc::vec![Call::Get(max)])
    );
    let broken = Phase::Failed(Fail::Broken, true);
    assert_eq!(walk(at(0, 10), &[Event::Got(Ok(0))]).0, broken, "empty");
    assert_eq!(walk(at(0, max + 10), &[Event::Got(Ok(GET_MAX + 1))]).0, broken, "over GET_MAX");
    assert_eq!(walk(at(5, 10), &[Event::Got(Ok(6))]).0, broken, "past the total");
    assert_eq!(walk(at(5, 10), &[Event::Got(Ok(5))]).0, Phase::Held);
    assert_eq!(
        walk(at(5, 10), &[Event::Tick]),
        (at(5, 10), alloc::vec![Call::Get(5)]),
        "next tick"
    );
    assert_eq!(walk(at(5, 10), &[Event::Got(Err(-22))]).0, Phase::Failed(Fail::Code(-22), true));
    let done = |total| {
        walk(Phase::Proving { stage: ISOLATION, out: true }, &[Event::Status(Ok((DONE, total)))])
    };
    assert_eq!(done(0).0, broken, "nothing to fetch");
    assert_eq!(done((1 << 20) + 1).0, broken, "over 1 MiB");
    assert_eq!(done(1 << 20), (at(0, 1 << 20), alloc::vec![Call::Get(0)]));
}
