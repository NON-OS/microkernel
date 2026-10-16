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

extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::sync::atomic::{AtomicU64, Ordering};

use super::loadavg_math::{fold, ONE, WEIGHTS};

static LOAD_1: AtomicU64 = AtomicU64::new(0);
static LOAD_5: AtomicU64 = AtomicU64::new(0);
static LOAD_15: AtomicU64 = AtomicU64::new(0);

pub fn read_loadavg() -> String {
    let (load1, load5, load15) = get_load_averages();
    let (running, total) = get_process_counts();
    let last_pid = crate::process::last_pid();
    format!("{:.2} {:.2} {:.2} {}/{} {}\n", load1, load5, load15, running, total, last_pid)
}

pub fn get_load_averages() -> (f64, f64, f64) {
    let [l1, l5, l15] = load_averages_fixed();
    let scale = ONE as f64;
    (l1 as f64 / scale, l5 as f64 / scale, l15 as f64 / scale)
}

pub fn load_averages_fixed() -> [u64; 3] {
    [
        LOAD_1.load(Ordering::Relaxed),
        LOAD_5.load(Ordering::Relaxed),
        LOAD_15.load(Ordering::Relaxed),
    ]
}

fn get_process_counts() -> (usize, usize) {
    let stats = crate::sched::get_scheduler_stats();
    (stats.running_count, stats.total_count)
}

/// Fold one sample of the runnable count into each window's average.
pub fn update_load_averages() {
    let now = crate::sched::get_runnable_count() as u64 * ONE;
    for (avg, w) in [&LOAD_1, &LOAD_5, &LOAD_15].into_iter().zip(WEIGHTS) {
        avg.store(fold(avg.load(Ordering::Relaxed), w, now), Ordering::Relaxed);
    }
}
