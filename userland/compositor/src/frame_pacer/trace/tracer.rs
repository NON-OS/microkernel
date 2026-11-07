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

//! The frame tracer the compositor carries. Off, every hook is one test of
//! a flag; on, each frame that drew anything is one line on the debug
//! channel (serial in a build with capsule serial output, and the
//! compositor's own inbox).

use super::calib::Calib;
use super::clock::{self, Source};
use super::line;
use super::record::FrameRecord;
use crate::state::damage::Rect;

pub struct Tracer {
    on: bool,
    src: Source,
    calib: Calib,
    per_ms: Option<u64>,
    frame: u32,
    rec: FrameRecord,
}

impl Tracer {
    pub const fn off() -> Self {
        Tracer {
            on: false,
            src: Source::Millis,
            calib: Calib { ticks0: 0, ms0: 0 },
            per_ms: None,
            frame: 0,
            rec: FrameRecord {
                waited_ms: None,
                wake: 0,
                start: 0,
                composite: 0,
                flip: 0,
                end: 0,
                rects: 0,
                pixels: 0,
                direct: 0,
            },
        }
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn set(&mut self, on: bool) {
        *self = Tracer::off();
        if on {
            self.on = true;
            self.src = clock::source();
            self.calib = Calib { ticks0: clock::now(self.src), ms0: clock::millis() };
            // Milliseconds are their own rate.
            self.per_ms = (self.src == Source::Millis).then_some(1);
            if let Some(r) = self.per_ms {
                emit(&line::header(self.src.name(), r));
            }
        }
    }

    /// A stamp, or 0 when off: what `composed` and `flipped` are handed.
    pub fn mark(&self) -> u64 {
        if self.on {
            clock::now(self.src)
        } else {
            0
        }
    }

    /// A request a frame will answer was received; it was queued at
    /// `queued_ms` on the uptime clock, and waited from then to now. A stamp
    /// past now (the two readings' rounding) is no wait; a stamp of zero is
    /// one the kernel did not give, and no wait is said.
    pub fn client(&mut self, queued_ms: u64) {
        if self.on {
            let now = clock::now(self.src);
            let waited = (queued_ms != 0).then(|| clock::millis().saturating_sub(queued_ms));
            self.rec.client(waited, now);
        }
    }

    pub fn begin(&mut self) {
        if self.on {
            self.rec.start = clock::now(self.src);
        }
    }

    pub fn composed(&mut self, since: u64, r: Rect) {
        if self.on {
            let spent = clock::now(self.src).saturating_sub(since);
            self.rec.composed(spent, r.width, r.height);
        }
    }

    pub fn flipped(&mut self, since: u64) {
        if self.on {
            self.rec.flipped(clock::now(self.src).saturating_sub(since));
        }
    }

    /// The frame scanned client buffer `handle` out with no composite.
    pub fn scanned(&mut self, handle: u64) {
        if self.on {
            self.rec.direct = handle;
        }
    }

    /// The frame is over: a line when it drew, and the next frame starts
    /// clean. Frames before the rate settled are not printed.
    pub fn end(&mut self) {
        if !self.on {
            return;
        }
        // A turn that drew nothing is not a frame.
        if self.rec.rects == 0 && self.rec.direct == 0 {
            self.rec = FrameRecord::default();
            return;
        }
        let now = clock::now(self.src);
        self.rec.end = now;
        if self.per_ms.is_none() {
            self.per_ms = self.calib.per_ms(now, clock::millis());
            if let Some(r) = self.per_ms {
                emit(&line::header(self.src.name(), r));
            }
        }
        if let Some(per_ms) = self.per_ms {
            let stages = self.rec.stages(per_ms);
            emit(&line::row(self.frame, &stages, self.rec.rects, self.rec.pixels, per_ms));
            if self.rec.direct != 0 {
                emit(&line::direct(self.frame, self.rec.direct));
            }
            self.frame = self.frame.wrapping_add(1);
        }
        self.rec = FrameRecord::default();
    }
}

fn emit(text: &str) {
    for l in text.split_inclusive('\n') {
        let _ = nonos_libc::mk_debug(l.as_ptr(), l.len());
    }
}
