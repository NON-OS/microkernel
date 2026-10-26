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

//! One step of the store's PROVE: from where it stands and what happened,
//! to where it stands next and the call to make. Once PROVE is answered with
//! a job the job is Reading, so any failure after it but ENOENT (no sealed
//! run) has used the run up.
//!
//! Pure; market_proofs holds it.

use super::proof_event::{Call, Event};
use super::proof_fail::Fail;
use super::proof_get::{fits, next};
use super::proof_said::ENOENT;
use super::proof_seq::{Phase, DONE, QUEUED};

const EPERM: i64 = -1;
const EBUSY: i64 = -16;

pub fn step(phase: Phase, event: Event) -> (Phase, Call) {
    use Phase::*;
    let failed = |f: Fail, past: bool| (Failed(f, past), Call::None);
    match (phase, event) {
        (Policy, Event::Tick) => (Policy, Call::Policy),
        (Policy, Event::Policy(Ok(()))) => (Boxed, Call::AskJob),
        (Policy, Event::Policy(Err(f))) => failed(f, false),
        (Boxed, Event::Job(0)) => (Asked, Call::Lookup),
        (Boxed, Event::Job(EPERM)) => failed(Fail::MayNot, false),
        (Boxed, Event::Job(EBUSY)) => failed(Fail::Busy, false),
        (Boxed, Event::Job(rc)) => failed(Fail::Code(rc as i32), false),
        (Asked, Event::Tick) => (Asked, Call::Lookup),
        (Asked, Event::Up) => (Up, Call::Prove),
        (Asked, Event::Late) => failed(Fail::NotStarted, false),
        (Up, Event::Proved(Ok(()))) => (Proving { stage: QUEUED, out: false }, Call::None),
        (Up, Event::Proved(Err(c))) if c == EBUSY as i32 => failed(Fail::Busy, false),
        (Up, Event::Proved(Err(c))) => failed(Fail::Code(c), false),
        (Proving { stage, out: false }, Event::Tick) => {
            (Proving { stage, out: true }, Call::Status)
        }
        (Proving { stage, .. }, Event::StatusLate) => (Proving { stage, out: false }, Call::None),
        (Proving { .. }, Event::Status(Ok((DONE, total)))) if fits(total) => {
            (Fetching { at: 0, total }, Call::Get(0))
        }
        (Proving { .. }, Event::Status(Ok((DONE, _)))) => failed(Fail::Broken, true),
        (Proving { .. }, Event::Status(Ok((s, _)))) if s < DONE => {
            (Proving { stage: s, out: false }, Call::None)
        }
        // Failed with a zero status, or a stage the wire does not have.
        (Proving { .. }, Event::Status(Ok(_))) => failed(Fail::Code(0), true),
        (Proving { .. }, Event::Status(Err(ENOENT))) => failed(Fail::Code(ENOENT), false),
        (Proving { .. }, Event::Status(Err(c))) => failed(Fail::Code(c), true),
        (Proving { .. } | Fetching { .. }, Event::Late) => failed(Fail::Quiet, true),
        (Fetching { at, total }, Event::Got(Ok(len))) => match next(at, total, len) {
            Some(end) if end == total => (Held, Call::None),
            Some(end) => (Fetching { at: end, total }, Call::Get(end)),
            None => failed(Fail::Broken, true),
        },
        (Fetching { .. }, Event::Got(Err(c))) => failed(Fail::Code(c), true),
        (Fetching { at, total }, Event::Tick) => (Fetching { at, total }, Call::Get(at)),
        (p, _) => (p, Call::None),
    }
}
