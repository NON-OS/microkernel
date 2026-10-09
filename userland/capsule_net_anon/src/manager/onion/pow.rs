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


//! Solving the service's puzzle before an introduction, a slice per idle
//! turn, so the capsule keeps answering requests while it works.
//!
//! Each introduction needs a fresh solution: the service keeps a replay
//! cache of nonces, so a solution sent through one introduction point is
//! refused if sent through the next. The effort rises after each point that
//! did not get us through, as hs_client.c raises it.
//!
//! Solving is bounded three ways. The effort is capped at
//! CLIENT_MAX_EFFORT. Each idle turn hashes for at most SLICE_MS and at most
//! MAX_CHUNKS * CHUNK indices, whichever ends first, so a stalled clock
//! cannot hold the loop. And a lookup solves only until SOLVE_SECONDS after
//! its first puzzle; past that it introduces without a solution, which the
//! service still accepts, at the back of its queue. Dropping the lookup,
//! because its stream closed or its deadline passed, drops the puzzle and the
//! solver's memory with it, which is the cancellation.

extern crate alloc;

use alloc::boxed::Box;

use nonos_libc::{crypto_random, mk_uptime_ms};

use crate::onion::pow::{effort, Puzzle};
use crate::trace;

use super::job::{OnionJob, PowState};

const SLICE_MS: u64 = 100;
const CHUNK: u32 = 4096;
const MAX_CHUNKS: u32 = 32;
/// How long one lookup may spend solving, from its first puzzle.
pub(super) const SOLVE_SECONDS: u64 = 90;

/// Whether the introduction may go out now: no puzzle is asked for, or one
/// is solved, or solving has run out of time. Otherwise runs one slice and
/// returns false.
pub(super) fn ready(job: &mut OnionJob, now: u64, step_seconds: u64) -> bool {
    let Some(params) = job.pow else {
        return true;
    };
    let effort = effort(params.suggested_effort, job.unreachable);
    if effort == 0 {
        return true;
    }
    match &job.pow_state {
        PowState::Solved(_) | PowState::Skipped => return true,
        PowState::Solving(puzzle) if puzzle.effort() == effort => {}
        _ => {
            if !start(job, &params, effort, now, step_seconds) {
                return true;
            }
        }
    }
    if job.solve_until.is_some_and(|until| now >= until) {
        trace::say(b"onion puzzle not solved in time, introducing without a solution");
        job.pow_state = PowState::Skipped;
        return true;
    }
    let PowState::Solving(puzzle) = &mut job.pow_state else {
        return true;
    };
    let began = mk_uptime_ms();
    for _ in 0..MAX_CHUNKS {
        if let Some(solution) = puzzle.step(CHUNK) {
            trace::say_num(b"onion puzzle solved, equi-x instances", u64::from(puzzle.solves) + 1);
            job.pow_state = PowState::Solved(solution);
            return true;
        }
        if mk_uptime_ms().saturating_sub(began) >= SLICE_MS as i64 {
            break;
        }
    }
    false
}

/// Set up a puzzle at `effort`. The first one of a lookup also fixes how
/// long it may solve and moves its deadline out to cover that. False when
/// there is nothing to solve with: no randomness for the nonce, or no memory
/// for the solver. The introduction then goes out without a solution, which
/// costs priority and nothing else.
fn start(job: &mut OnionJob, params: &crate::onion::pow::PowParams, effort: u32, now: u64, step_seconds: u64) -> bool {
    job.pow_state = PowState::Skipped;
    let mut nonce = [0u8; 16];
    if crypto_random(nonce.as_mut_ptr(), nonce.len()) != nonce.len() as i64 {
        trace::say(b"onion puzzle has no nonce, introducing without a solution");
        return false;
    }
    let Some(puzzle) = Puzzle::new(&job.lookup.blinded, params, effort, nonce) else {
        trace::say(b"onion puzzle solver has no memory, introducing without a solution");
        return false;
    };
    if job.solve_until.is_none() {
        let until = now.saturating_add(SOLVE_SECONDS);
        job.solve_until = Some(until);
        /* Room after solving for one introduction and the service's answer. */
        job.deadline = job.deadline.max(until.saturating_add(2 * step_seconds));
    }
    trace::say_num(b"onion service asks for a puzzle, effort", u64::from(effort));
    job.pow_state = PowState::Solving(Box::new(puzzle));
    true
}
