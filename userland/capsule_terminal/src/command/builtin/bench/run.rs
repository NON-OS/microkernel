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

//! Taking the measurements and reporting them.

use nonos_bench_core::{measure, Overhead, Summary};

use super::format::{heading, row};
use super::probes::{fire, Probe, CALIBRATION, PROBES, SAMPLES};
use crate::command::output::Output;

pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) {
    let overhead = Overhead::measure(CALIBRATION);
    out.writeln(b"cycles per operation, counter overhead already subtracted.");
    out.writeln(b"percentiles are nearest rank: every figure below was measured.");
    blank_line(out, overhead.cycles);
    heading(out);

    // One buffer, reused. A capsule has no business allocating sixteen kilobytes
    // per probe when the samples are consumed before the next probe starts.
    let mut buf = [0u64; SAMPLES];
    for (i, p) in PROBES.iter().enumerate() {
        let s: Summary = measure(&mut buf, overhead, || fire(i));
        row(out, p.name, &s);
    }
    for p in PROBES.iter() {
        note(out, p);
    }
}

fn blank_line(out: &mut Output<'_>, overhead: u64) {
    let mut line = [b' '; 48];
    let head = b"counter overhead: ";
    line[..head.len()].copy_from_slice(head);
    let mut b = [0u8; 20];
    let n = super::format::decimal(overhead, &mut b);
    line[head.len()..head.len() + n].copy_from_slice(&b[..n]);
    out.writeln(&line);
}

fn note(out: &mut Output<'_>, p: &Probe) {
    let mut line = [b' '; 96];
    line[..p.name.len()].copy_from_slice(p.name);
    let at = 10;
    let end = (at + p.what.len()).min(line.len());
    line[at..end].copy_from_slice(&p.what[..end - at]);
    out.writeln(&line);
}
