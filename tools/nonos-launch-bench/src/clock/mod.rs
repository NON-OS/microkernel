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

//! A clock read: libc's clock-page reader with RDTSC (the after), against
//! the parts of the MkTimeMonotonic syscall that run on the host (the gate,
//! from the gate section, and the handler's own counter read and scaling).
//! The syscall's trap in and out is not on the host and not in `before`.

#[allow(dead_code)]
#[path = "../../../../src/sys/clock/page/layout.rs"]
mod layout;
#[allow(dead_code)]
#[path = "../../../../userland/libc/src/time/clock_page.rs"]
mod reader;

use std::hint::black_box;
use std::sync::atomic::AtomicU64;
use std::time::Instant;

const READS: u32 = 5_000_000;

fn rdtsc() -> u64 {
    // SAFETY: RDTSC has no side effect and is readable at CPL 3 on the host.
    unsafe { core::arch::x86_64::_rdtsc() }
}

fn best(f: impl Fn() -> u64) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..5 {
        let t = Instant::now();
        let mut acc = 0u64;
        for _ in 0..READS {
            acc = acc.wrapping_add(f());
        }
        black_box(acc);
        best = best.min(t.elapsed().as_nanos() as f64 / f64::from(READS));
    }
    best
}

pub fn run(gate_ns: f64) {
    let page: Vec<AtomicU64> = (0..layout::WORDS).map(|_| AtomicU64::new(0)).collect();
    let boot = rdtsc();
    layout::publish(&page, &layout::Values { flags: layout::FLAG_USER_COUNTER, hz: 2_100_000_000, boot_count: boot });
    let page_read = best(|| reader::read(black_box(&page), rdtsc).unwrap_or(0));
    let handler = best(|| layout::ms_since(rdtsc(), black_box(boot), black_box(2_100_000_000)));
    println!("# clock read, best of 5 x {READS}");
    println!("{:<44} {page_read:>8.1} ns  host", "clock page read (libc reader + rdtsc)");
    println!("{:<44} {handler:>8.1} ns  host", "MkTimeMonotonic handler body");
    println!("{:<44} {:>8.1} ns  modelled (trap not included)", "MkTimeMonotonic gate + handler", gate_ns + handler);
}
