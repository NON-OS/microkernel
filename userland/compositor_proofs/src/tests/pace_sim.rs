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

//! A model of the compositor's turn over the kernel's receive, to compare
//! drain rules by when each input event's frame is done. Times in us. A
//! timed receive that finds nothing sleeps until a delivery or the first
//! timer sweep at or after its deadline (ipc/wait_rule.rs and
//! recv_from_drain.rs; the sweep runs at the 100 Hz tick); NO_WAIT looks once.

pub const TICK_US: u64 = 10_000;
const FIRST_WAIT_MS: u64 = 16;
const MAX_BATCH: usize = 16;
const DISPATCH_US: u64 = 50;
const COMPOSE_US: u64 = 1_000;
pub const NO_WAIT: u64 = u64::MAX;

/// What the drain waits after it has served one request.
pub type Rule = fn(now_ms: i64, composed_at_ms: i64) -> u64;

pub struct Run {
    /// Event to the end of the frame it is first shown in, each event.
    pub latency_us: Vec<u64>,
    /// When each frame that drew some input started composing.
    pub frames_at_us: Vec<u64>,
}

fn sweep_after(at: u64) -> u64 {
    at.div_ceil(TICK_US) * TICK_US
}

/// One receive at `now` over `arrivals[next..]`: the time it returns and
/// whether it took the next event.
fn receive(now: u64, timeout_ms: u64, arrivals: &[u64], next: usize) -> (u64, bool) {
    let queued = arrivals.get(next).copied();
    if queued.is_some_and(|at| at <= now) {
        return (now, true);
    }
    if timeout_ms == NO_WAIT {
        return (now, false);
    }
    let woken = sweep_after(now + timeout_ms * 1000);
    match queued {
        Some(at) if at < woken => (at, true),
        _ => (woken, false),
    }
}

pub fn run(rule: Rule, arrivals: &[u64]) -> Run {
    let (mut now, mut next, mut composed_at) = (0u64, 0usize, 0u64);
    let mut pending: Vec<u64> = Vec::new();
    let mut out = Run { latency_us: Vec::new(), frames_at_us: Vec::new() };
    while next < arrivals.len() || !pending.is_empty() {
        for i in 0..MAX_BATCH {
            let ms = |us: u64| (us / 1000) as i64;
            let timeout = if i == 0 { FIRST_WAIT_MS } else { rule(ms(now), ms(composed_at)) };
            let (at, took) = receive(now, timeout, arrivals, next);
            now = at;
            if !took {
                break;
            }
            pending.push(arrivals[next]);
            next += 1;
            now += DISPATCH_US;
        }
        if !pending.is_empty() {
            composed_at = now;
            out.frames_at_us.push(now);
            now += COMPOSE_US;
            out.latency_us.extend(pending.drain(..).map(|a| now - a));
        }
    }
    out
}

/// `count` events `period_us` apart, the first at `start_us`.
pub fn every(period_us: u64, start_us: u64, count: usize) -> Vec<u64> {
    (0..count as u64).map(|i| start_us + i * period_us).collect()
}

pub fn mean(v: &[u64]) -> u64 {
    v.iter().sum::<u64>() / v.len().max(1) as u64
}

/// What a paced rule is given: now, when the last passes served input and
/// drew, and whether the drain holds damage (server/runner/pace.rs).
pub type PacedRule = fn(now_ms: i64, served_ms: i64, drew_ms: i64, damaged: bool) -> u64;

pub struct Mixed {
    /// Event to the end of its frame, for each event that draws.
    pub draw_latency_us: Vec<u64>,
    /// When each pass that drew started composing.
    pub frames_at_us: Vec<u64>,
    /// When each pass that served events and drew nothing ended: a pointer
    /// moved on the cursor plane.
    pub plane_at_us: Vec<u64>,
}

/// The same turn over `(arrival, draws)` events, with entry.rs's two
/// stamps: a pass that served input or drew, and a pass that drew.
pub fn run_mixed(rule: PacedRule, arrivals: &[(u64, bool)]) -> Mixed {
    let times: Vec<u64> = arrivals.iter().map(|&(at, _)| at).collect();
    let (mut now, mut next) = (0u64, 0usize);
    let (mut served_at, mut drew_at) = (0u64, 0u64);
    let mut out =
        Mixed { draw_latency_us: Vec::new(), frames_at_us: Vec::new(), plane_at_us: Vec::new() };
    while next < arrivals.len() {
        let mut drawn: Vec<u64> = Vec::new();
        let mut served = false;
        for i in 0..MAX_BATCH {
            let ms = |us: u64| (us / 1000) as i64;
            let timeout = if i == 0 {
                FIRST_WAIT_MS
            } else {
                rule(ms(now), ms(served_at), ms(drew_at), !drawn.is_empty())
            };
            let (at, took) = receive(now, timeout, &times, next);
            now = at;
            if !took {
                break;
            }
            served = true;
            if arrivals[next].1 {
                drawn.push(arrivals[next].0);
            }
            next += 1;
            now += DISPATCH_US;
        }
        if served || !drawn.is_empty() {
            served_at = now;
        }
        if drawn.is_empty() {
            if served {
                out.plane_at_us.push(now);
            }
            continue;
        }
        drew_at = now;
        out.frames_at_us.push(now);
        now += COMPOSE_US;
        out.draw_latency_us.extend(drawn.drain(..).map(|a| now - a));
    }
    out
}

/// A title-bar drag's steps, `period_us` apart from `start_us`: each the
/// pointer moved on the cursor plane, then the window's scene submit
/// `behind_us` later.
pub fn drag_steps(period_us: u64, start_us: u64, behind_us: u64, count: usize) -> Vec<(u64, bool)> {
    every(period_us, start_us, count)
        .into_iter()
        .flat_map(|at| [(at, false), (at + behind_us, true)])
        .collect()
}
