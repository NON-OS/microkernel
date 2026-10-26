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

//! What ends a run before the prover reads it, the kernel's answers and the
//! job's, nothing moving a run that has ended, and the question's words.

use super::proof_event::{Call, Event};
use super::proof_fail::Fail;
use super::proof_said::STOPPED;
use super::proof_seq::{Phase, DEVICE};
use super::proof_words::{question, working, CHECKED, NOT_NOW, PROVE_IT, READY};
use super::prove_seq::walk;

#[test]
fn the_kernel_and_the_job_refusing_end_it_before_the_run_is_read() {
    let job = |rc| walk(Phase::Boxed, &[Event::Job(rc)]).0;
    assert_eq!(job(-1), Phase::Failed(Fail::MayNot, false));
    assert_eq!(job(-16), Phase::Failed(Fail::Busy, false));
    assert_eq!(job(-12), Phase::Failed(Fail::Code(-12), false));
    let late = walk(Phase::Asked, &[Event::Tick, Event::Late]);
    assert_eq!(
        late,
        (Phase::Failed(Fail::NotStarted, false), alloc::vec![Call::Lookup, Call::None])
    );
    assert_eq!(walk(Phase::Up, &[Event::Proved(Err(-16))]).0, Phase::Failed(Fail::Busy, false));
    assert_eq!(
        walk(Phase::Up, &[Event::Proved(Err(-12))]).0,
        Phase::Failed(Fail::Code(-12), false)
    );
    let proving = Phase::Proving { stage: DEVICE, out: true };
    assert_eq!(walk(proving, &[Event::Status(Err(-2))]).0, Phase::Failed(Fail::Code(-2), false));
    assert_eq!(
        walk(proving, &[Event::Status(Err(-1103))]).0,
        Phase::Failed(Fail::Code(-1103), true)
    );
    let failed = Phase::Failed(Fail::Quiet, true);
    assert_eq!(walk(failed, &[Event::Tick, Event::Up, Event::Got(Ok(1))]).0, failed);
}

#[test]
fn the_question_and_what_it_comes_to_are_the_notes_words() {
    let want = "Stopped. The proof is thrown away; this run cannot be proven again";
    assert_eq!(STOPPED, want.as_bytes());
    assert_eq!(question(b"age"), b"Prove how age ran?");
    let checked =
        "Checked on this machine against the box it was sold with. Nothing leaves this machine.";
    assert_eq!(
        (CHECKED, PROVE_IT, NOT_NOW),
        (checked.as_bytes(), &b"Prove it"[..], &b"Not now"[..])
    );
    assert_eq!(READY, b"Proof ready: held here, not sent yet");
    assert_eq!(
        working(Phase::Proving { stage: DEVICE, out: true }),
        Some(&b"Proving this device"[..])
    );
    assert_eq!(working(Phase::Policy), Some(&b"Starting the prover"[..]));
    assert_eq!(working(Phase::Held), None);
}
