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

use super::phase::Phase;
use super::record::{Record, ORDER};

pub(crate) const LINE_MAX: usize = 384;

/*
 * The serial line for one launch, built in a buffer so it goes out in a
 * single write:
 *
 *   [LAUNCH] app.clock pid=12 cert=410 manifest=388 attest=- create=95
 *   elf=1210 install=60 queue=12 syscall=140 surface=2210 frame=8120
 *   total=12645 end=frame
 *
 * Spans are microseconds from the previous phase reached; `-` is a phase
 * the launch did not reach. `end` says why the line was written.
 */
pub(crate) struct Text {
    buf: [u8; LINE_MAX],
    len: usize,
}

impl Text {
    pub(crate) const fn new() -> Self {
        Self { buf: [0; LINE_MAX], len: 0 }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    fn str(&mut self, s: &[u8]) {
        for &b in s {
            if self.len < LINE_MAX {
                self.buf[self.len] = b;
                self.len += 1;
            }
        }
    }

    fn dec(&mut self, mut v: u64) {
        let mut d = [0u8; 20];
        let mut n = 0;
        loop {
            d[n] = b'0' + (v % 10) as u8;
            n += 1;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        while n > 0 {
            n -= 1;
            self.str(&d[n..n + 1]);
        }
    }
}

pub(crate) fn label(phase: Phase) -> &'static [u8] {
    match phase {
        Phase::Begin => b"begin",
        Phase::CertVerified => b"cert",
        Phase::ManifestVerified => b"manifest",
        Phase::Attested => b"attest",
        Phase::ProcessCreated => b"create",
        Phase::ImageLoaded => b"elf",
        Phase::Installed => b"install",
        Phase::Queued => b"queue",
        Phase::FirstSyscall => b"syscall",
        Phase::FirstSurface => b"surface",
        Phase::FirstFrame => b"frame",
    }
}

pub(crate) fn render(r: &Record, end: &[u8], out: &mut Text) {
    out.str(b"[LAUNCH] ");
    out.str(r.name());
    out.str(b" pid=");
    out.dec(u64::from(r.pid()));
    for phase in ORDER.iter().skip(1) {
        out.str(b" ");
        out.str(label(*phase));
        out.str(b"=");
        match r.span_ns(*phase) {
            Some(ns) => out.dec(ns / 1000),
            None => out.str(b"-"),
        }
    }
    out.str(b" total=");
    out.dec(r.total_ns() / 1000);
    out.str(b" end=");
    out.str(end);
}
