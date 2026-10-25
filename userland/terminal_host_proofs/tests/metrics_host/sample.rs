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

//! The empty sample.

use crate::rail::mem::{summarize, Mem};
use crate::rail::metrics::{Proc, Sample, MAX_PROCS};
use crate::rail::value::Metric;

#[test]
fn an_empty_sample_has_no_live_processes() {
    let s = Sample::EMPTY;
    assert_eq!(s.live().len(), 0);
    const { assert!(MAX_PROCS >= 40) };
    // Nothing in the empty sample claims a reading.
    let p = s.procs[0];
    assert_eq!((p.pid, p.cpu_pct, p.mem_kb, p.run_ticks), (0, 0, 0, 0));
    assert_eq!((s.total_ticks, s.mem_total_kb, s.cpu_pct, s.uptime_ms), (0, 0, 0, 0));
    assert_eq!((s.mem, s.disk.used_kb), (Mem::UNKNOWN, Metric::Unknown));
    assert_eq!(s.load_avg.value(), None);
    assert_eq!(s.net.name_str(), "net0");
    // With no live process the resident sum stays unmeasured; a total still shows.
    let m = summarize(s.live(), 2048);
    assert_eq!((m.used_kb, m.total_kb), (Metric::Unknown, Metric::Known(2048)));
    let one = Proc { mem_kb: 512, ..Proc::EMPTY };
    assert_eq!(summarize(&[one], 0).used_kb.value(), Some(512));
}
