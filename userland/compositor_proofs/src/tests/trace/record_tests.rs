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

use crate::frame_pacer::trace::calib::{Calib, SETTLE_MS};
use crate::frame_pacer::trace::line::{header, ns, row, COLUMNS};
use crate::frame_pacer::trace::record::FrameRecord;

/// Ticks a millisecond in these frames.
const PER_MS: u64 = 100;

fn frame() -> FrameRecord {
    let mut r = FrameRecord { start: 1_000, ..FrameRecord::default() };
    r.client(Some(3), 900);
    r.composed(300, 10, 20);
    r.composed(200, 5, 5);
    r.flipped(100);
    r.end = 1_700;
    r
}

#[test]
fn the_first_request_of_a_frame_names_its_latency() {
    let mut r = frame();
    r.client(Some(1), 950);
    assert_eq!((r.waited_ms, r.wake), (Some(3), 900), "a later request waited less");
}

#[test]
fn the_stages_add_up_to_the_frame() {
    let s = frame().stages(PER_MS);
    assert_eq!(s.ready, Some(300));
    assert_eq!(s.wake, Some(100));
    assert_eq!((s.composite, s.flip), (500, 100));
    assert_eq!(s.damage, 700 - 500 - 100);
    assert_eq!(s.total, 1_700 - 600);
}

#[test]
fn a_request_taken_as_it_was_queued_did_not_wait() {
    let mut r = frame();
    (r.waited_ms, r.wake) = (None, 0);
    r.client(Some(0), 900);
    let s = r.stages(PER_MS);
    assert_eq!(s.ready, Some(0));
    assert_eq!(s.total, 1_700 - 900, "the frame starts at the wake");
}

#[test]
fn a_wait_longer_than_the_counter_has_run_starts_the_frame_at_zero() {
    let mut r = FrameRecord { start: 250, end: 400, ..FrameRecord::default() };
    r.client(Some(5), 200);
    let s = r.stages(PER_MS);
    assert_eq!(s.ready, Some(500), "the wait is said as it was");
    assert_eq!(s.total, 400, "and the frame counted from the counter's zero");
}

// The cycle counter's rate is measured once, over the trace's first
// 100 ms, so it is a little off. A request that waited 2 ms, a minute
// into a trace, at 3 GHz measured 0.1% slow: placed on the counter by
// that rate from the trace's start, its queued millisecond came 60 ms
// early and the frame read a 62 ms wait (measured 0.1% fast it came
// after its own receipt and read none). Kept as the milliseconds it
// waited, it reads its 2 ms, the rate's error on 2 ms alone.
#[test]
fn a_wait_does_not_grow_with_the_time_since_tracing_began() {
    let (truth, measured) = (3_000_000u64, 2_997_000u64);
    let (ticks0, ms0) = (5_000u64, 40u64);
    let (queued_ms, wake_ms) = (ms0 + 60_000, ms0 + 60_002);
    let wake = ticks0 + (wake_ms - ms0) * truth;
    let placed = ticks0 + (queued_ms - ms0) * measured;
    let read_before = (wake - placed) / measured;
    assert_eq!(read_before, 62, "the old reading");
    let mut r = FrameRecord { start: wake + truth, end: wake + 2 * truth, ..Default::default() };
    r.client(Some(wake_ms - queued_ms), wake);
    let ready = r.stages(measured).ready.expect("a request led");
    assert_eq!(ready / measured, 2);
    let mut early = FrameRecord { start: 2_000 + truth, end: 2_000 + 2 * truth, ..r };
    early.wake = 2_000;
    assert_eq!(early.stages(measured).ready, Some(ready), "the same a moment into the trace");
}

#[test]
fn a_request_with_no_queued_stamp_has_a_wake_and_no_wait() {
    let mut r = FrameRecord { start: 1_000, end: 1_200, ..FrameRecord::default() };
    r.client(None, 900);
    let s = r.stages(PER_MS);
    assert_eq!((s.ready, s.wake, s.total), (None, Some(100), 300));
}

#[test]
fn a_frame_no_request_led_has_no_client_stages() {
    let mut r = frame();
    r.wake = 0;
    let s = r.stages(PER_MS);
    assert_eq!((s.ready, s.wake), (None, None));
    assert_eq!(s.total, 700);
}

#[test]
fn rects_and_pixels_are_counted() {
    let r = frame();
    assert_eq!((r.rects, r.pixels), (2, 225));
}

#[test]
fn the_rate_waits_to_settle() {
    let c = Calib { ticks0: 1_000, ms0: 50 };
    assert_eq!(c.per_ms(1_000 + 3_000 * 99, 50 + SETTLE_MS - 1), None);
    assert_eq!(c.per_ms(1_000 + 3_000 * SETTLE_MS, 50 + SETTLE_MS), Some(3_000));
    assert_eq!(c.per_ms(999, 50 + SETTLE_MS), None, "a counter that went back");
    assert_eq!(c.per_ms(2_000, 40), None, "a clock that went back");
}

#[test]
fn ticks_become_nanoseconds_without_overflow() {
    assert_eq!(ns(3_000, 3_000), 1_000_000);
    assert_eq!(ns(1, 3_000), 333);
    assert_eq!(ns(u64::MAX, 1), u64::MAX);
    assert_eq!(ns(5, 0), 0);
}

#[test]
fn a_row_leaves_stages_the_frame_did_not_have_empty() {
    let mut r = frame();
    r.waited_ms = None;
    let s = r.stages(PER_MS);
    let line = row(4, &s, 2, 225, 1_000_000);
    assert_eq!(line, "[FTRACE] 4,,100,100,500,100,800,2,225\n");
    let fields = line.trim_end().trim_start_matches("[FTRACE] ").split(',').count();
    assert_eq!(fields, COLUMNS.split(',').count());
}

#[test]
fn the_header_names_the_clock_and_the_columns() {
    let h = header("tsc", 2_100_000);
    assert_eq!(h, format!("[FTRACE] # clock=tsc ticks_per_ms=2100000\n[FTRACE] {COLUMNS}\n"));
    assert!(h.lines().all(|l| l.len() < 256), "each line fits one debug call");
}
