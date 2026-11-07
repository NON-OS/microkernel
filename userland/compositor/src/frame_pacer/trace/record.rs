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

//! One frame's stamps. Pure, so the host harness keeps the same record the
//! capsule does and both print the same line.
//!
//! A request's time in the inbox is kept as the milliseconds between its
//! queueing and its receipt, both on the uptime clock. It used to be kept as
//! the queued millisecond and put on the cycle counter at the frame's end by
//! the rate measured once, over the trace's first 100 ms: that rate's error
//! grew with every millisecond after it, so a frame a minute into a trace
//! could read tens of milliseconds of waiting it never had, or none of a wait
//! it had, and its total with it.

/// One frame, in clock ticks. A stamp of zero was not taken this frame.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct FrameRecord {
    /// How long the first request this frame answers waited in the inbox,
    /// in uptime milliseconds from its queueing to its receipt: the client's
    /// picture was ready that long before the wake. None when no request led.
    pub waited_ms: Option<u64>,
    /// When the compositor received that request.
    pub wake: u64,
    /// When the frame began.
    pub start: u64,
    /// Ticks spent composing, summed over the frame's rectangles.
    pub composite: u64,
    /// Ticks spent handing composed rectangles to the display.
    pub flip: u64,
    /// When the frame ended.
    pub end: u64,
    pub rects: u32,
    pub pixels: u64,
    /// The client buffer the plane scanned out this frame, 0 when none:
    /// the frame composed nothing but the band.
    pub direct: u64,
}

/// The stages of one frame, in ticks.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Stages {
    /// Queued to received; None when no client request led the frame.
    pub ready: Option<u64>,
    /// Received to the frame's start.
    pub wake: Option<u64>,
    /// The frame less its compose and flip: draining and merging damage,
    /// the band and lens checks, the cursor plane.
    pub damage: u64,
    pub composite: u64,
    pub flip: u64,
    /// The first stamp taken to the end.
    pub total: u64,
}

impl FrameRecord {
    /// The first request of the frame names its client stamps; later ones
    /// in the same frame waited less and are not the frame's latency.
    pub fn client(&mut self, waited_ms: Option<u64>, wake: u64) {
        if self.wake == 0 {
            self.waited_ms = waited_ms;
            self.wake = wake;
        }
    }

    pub fn composed(&mut self, ticks: u64, width: u32, height: u32) {
        self.composite = self.composite.saturating_add(ticks);
        self.rects = self.rects.saturating_add(1);
        self.pixels = self.pixels.saturating_add(width as u64 * height as u64);
    }

    pub fn flipped(&mut self, ticks: u64) {
        self.flip = self.flip.saturating_add(ticks);
    }

    /// The stages at `per_ms` ticks a millisecond. The inbox wait is the
    /// milliseconds it took, made ticks: whatever the rate's error, it is
    /// that error on a few milliseconds, never on the time since tracing
    /// began. A wait shows only with the wake it ended in.
    pub fn stages(&self, per_ms: u64) -> Stages {
        let wake = (self.wake != 0).then_some(self.wake);
        let ready = wake.and(self.waited_ms).map(|ms| ms.saturating_mul(per_ms));
        let spent = self.end.saturating_sub(self.start);
        let queued = ready.zip(wake).map(|(r, w)| w.saturating_sub(r));
        let first = [queued, wake].into_iter().flatten().fold(self.start, u64::min);
        Stages {
            ready,
            wake: wake.map(|w| self.start.saturating_sub(w)),
            damage: spent.saturating_sub(self.composite).saturating_sub(self.flip),
            composite: self.composite,
            flip: self.flip,
            total: self.end.saturating_sub(first),
        }
    }
}
