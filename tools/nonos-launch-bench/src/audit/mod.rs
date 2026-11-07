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

//! The syscall audit record, four threads recording at once: the one shared
//! ring behind a spin lock (as before) against the per-CPU rings (the
//! kernel's `percpu.rs`). The record is the kernel's shape; the clock read
//! and current-process lookup around it are the same before and after and
//! are not timed.

#[allow(dead_code)]
#[path = "../../../../src/syscall/dispatch/audit/percpu.rs"]
mod percpu;

use std::hint::black_box;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Instant;

use spin::Mutex;

#[derive(Clone)]
#[allow(dead_code)]
struct Entry {
    timestamp_ms: u64,
    syscall_num: u64,
    syscall_name: &'static str,
    pid: u32,
    result: i64,
    args: [u64; 4],
    success: bool,
}

fn entry(i: u64) -> Entry {
    Entry { timestamp_ms: i, syscall_num: 0x4c43_494d, syscall_name: "MkIpcCall", pid: 7, result: 0, args: [i; 4], success: true }
}

struct Shared {
    entries: [Option<Entry>; 256],
    head: usize,
    count: usize,
}

const THREADS: usize = 4;
const EACH: u64 = 500_000;

fn timed(f: impl Fn(usize, u64) + Send + Sync + 'static) -> f64 {
    let f = Arc::new(f);
    let start = Arc::new(Barrier::new(THREADS + 1));
    let handles: Vec<_> = (0..THREADS)
        .map(|cpu| {
            let (f, start) = (f.clone(), start.clone());
            thread::spawn(move || {
                start.wait();
                for i in 0..EACH {
                    f(cpu, black_box(i));
                }
            })
        })
        .collect();
    start.wait();
    let t = Instant::now();
    for h in handles {
        let _ = h.join();
    }
    t.elapsed().as_nanos() as f64 / (THREADS as f64 * EACH as f64)
}

pub fn run() {
    println!("# syscall audit record, {THREADS} threads at once, {EACH} records each (best of 3)");
    let mut before = f64::MAX;
    let mut after = f64::MAX;
    for _ in 0..3 {
        let shared = Arc::new(Mutex::new(Shared { entries: std::array::from_fn(|_| None), head: 0, count: 0 }));
        before = before.min(timed(move |_, i| {
            let mut log = shared.lock();
            let h = log.head;
            log.entries[h] = Some(entry(i));
            log.head = (h + 1) % 256;
            if log.count < 256 {
                log.count += 1;
            }
        }));
        let rings: Arc<percpu::PerCpuLog<Entry, 256>> = Arc::new(percpu::PerCpuLog::new());
        after = after.min(timed(move |cpu, i| assert!(rings.push(cpu, entry(i)), "a ring the heap had no room for")));
    }
    println!("{:<44} before={before:>8.1} ns  after={after:>8.1} ns  host", "audit record per call");
}
