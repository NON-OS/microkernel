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

//! The drain's follow-up wait (server/runner/pace.rs): a served request is
//! composed at once when the last frame is a floor old, input inside the
//! floor waits out its rest, and over the kernel's tick-woken receive that
//! takes a lone event's frame from up to a tick late to at once while a fast
//! stream of input is still composed at most once a floor.

use super::pace_sim::{every, mean, run, NO_WAIT, TICK_US};
use crate::runner::pace::{follow_up, FRAME_FLOOR_MS};

const DRAIN: &str = include_str!("../../../compositor/src/server/runner/drain.rs");
const ENTRY: &str = include_str!("../../../compositor/src/server/runner/entry.rs");
const LIBC_RECV: &str = include_str!("../../../libc/src/ipc/recv_from.rs");
const KERNEL_WAIT: &str = include_str!("../../../../src/syscall/microkernel/ipc/wait_rule.rs");

fn new_rule(now: i64, composed_at: i64) -> u64 {
    follow_up(now, composed_at).unwrap_or(NO_WAIT)
}

/// What the drain did before: every follow-up a 1 ms timed receive.
fn old_rule(_: i64, _: i64) -> u64 {
    1
}

#[test]
fn a_frame_a_floor_old_or_more_only_looks() {
    for since in [FRAME_FLOOR_MS, FRAME_FLOOR_MS + 1, 1000, i64::MAX / 2] {
        assert_eq!(follow_up(since, 0), None, "since {since}");
    }
    assert_eq!(follow_up(i64::MAX, i64::MIN), None);
}

#[test]
fn inside_the_floor_it_waits_exactly_the_rest() {
    for since in 0..FRAME_FLOOR_MS {
        let wait = follow_up(5000 + since, 5000);
        assert_eq!(wait, Some((FRAME_FLOOR_MS - since) as u64), "since {since}");
        assert!(wait.is_some_and(|w| w >= 1 && w <= FRAME_FLOOR_MS as u64));
    }
}

#[test]
fn a_clock_that_went_back_waits_the_floor_and_never_forever() {
    assert_eq!(follow_up(10, 20), Some(FRAME_FLOOR_MS as u64));
    assert_eq!(follow_up(i64::MIN, i64::MAX), Some(FRAME_FLOOR_MS as u64));
    for (now, at) in [(0, 0), (3, 1), (-5, 7), (i64::MIN, 0)] {
        assert_ne!(follow_up(now, at), Some(0), "0 would wait for ever");
    }
}

#[test]
fn the_drain_waits_as_pace_says_and_entry_passes_the_last_pass() {
    assert!(DRAIN.contains(
        "follow_up(now, paced_from(last, !ctx.damage.is_empty())).unwrap_or(MK_IPC_NO_WAIT)"
    ));
    assert!(!DRAIN.contains("RECV_POLL_MS"), "the 1 ms poll is gone");
    assert!(DRAIN.contains("if i == 0 {\n            RECV_FRAME_MS"));
    assert!(ENTRY.contains("drain_ipc(&mut ctx, &mut rx, &mut tx, last)"));
    let tick = ENTRY.find("frame_pacer::tick(&mut ctx);").expect("tick");
    let drew = ENTRY.find("let drew = !ctx.damage.is_empty();").expect("damage read");
    let served = ENTRY.find("if request.is_some() || drew {\n            last.served = t2;");
    let drawn = ENTRY.find("if drew {\n            last.drew = t2;");
    assert!(drew < tick, "damage is read before the pass composes");
    for gate in [served, drawn] {
        assert!(gate.is_some_and(|at| drew < at && at < tick), "both stamps before the frame");
    }
    assert_eq!(ENTRY.matches("last.served = t2").count(), 1);
    assert_eq!(ENTRY.matches("last.drew = t2").count(), 1);
    assert!(ENTRY.contains("let t2 = mk_uptime_ms();"));
}

#[test]
fn no_wait_is_the_kernels_look_once() {
    assert!(LIBC_RECV.contains("pub const MK_IPC_NO_WAIT: u64 = u64::MAX;"));
    assert!(KERNEL_WAIT.contains("pub(crate) const NO_WAIT: u64 = u64::MAX;"));
    assert!(KERNEL_WAIT.contains("if timeout_ms == NO_WAIT {\n        return Next::Again;"));
}

#[test]
fn a_lone_event_is_composed_at_once_not_a_tick_late() {
    // Events 200 ms apart at every phase of the tick.
    for phase in (0..TICK_US).step_by(700) {
        let at = every(200_000, 50_000 + phase, 20);
        let old = run(old_rule, &at);
        let new = run(new_rule, &at);
        let worst = *new.latency_us.iter().max().expect("events");
        assert!(worst <= 1_100, "phase {phase}: {worst} us");
        assert!(mean(&new.latency_us) < mean(&old.latency_us), "phase {phase}");
    }
}

#[test]
fn a_stream_is_composed_at_most_once_a_floor_and_never_later_than_before() {
    for hz in [60u64, 125, 250, 500, 1000] {
        let at = every(1_000_000 / hz, 3_300, (hz * 2) as usize);
        let old = run(old_rule, &at);
        let new = run(new_rule, &at);
        assert!(mean(&new.latency_us) <= mean(&old.latency_us), "{hz} Hz");
        let worst_new = *new.latency_us.iter().max().expect("events");
        let worst_old = *old.latency_us.iter().max().expect("events");
        assert!(worst_new <= worst_old, "{hz} Hz: {worst_new} > {worst_old}");
        // Times are read in whole ms, so a gap can be up to 1 ms short.
        let floor_us = (FRAME_FLOOR_MS as u64 - 1) * 1000;
        for pair in new.frames_at_us.windows(2) {
            assert!(pair[1] - pair[0] >= floor_us, "{hz} Hz: {:?}", pair);
        }
    }
}

/// Prints the numbers this benchmark reports:
/// cargo test --release pace_numbers -- --ignored --nocapture
#[test]
#[ignore]
fn pace_numbers() {
    let lone = every(200_000, 53_300, 200);
    let cases = [("lone event", lone)];
    let streams = [60u64, 125, 250, 1000].map(|hz| (hz, every(1_000_000 / hz, 3_300, 2000)));
    let line = |name: &str, at: &[u64]| {
        for (rule, f) in [("before", old_rule as fn(i64, i64) -> u64), ("after", new_rule)] {
            let r = run(f, at);
            let span = at.last().copied().unwrap_or(0) - at[0];
            let fps = (r.frames_at_us.len() as u64 * 1_000_000).checked_div(span).unwrap_or(0);
            println!(
                "{name} {rule}: mean {} us, worst {} us, {} frames/s",
                mean(&r.latency_us),
                r.latency_us.iter().max().copied().unwrap_or(0),
                fps
            );
        }
    };
    for (name, at) in &cases {
        line(name, at);
    }
    for (hz, at) in &streams {
        line(&format!("{hz} Hz motion"), at);
    }
}
