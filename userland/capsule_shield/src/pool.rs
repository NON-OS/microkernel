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

//! The threads the prover proves on.
//!
//! nox_prover runs its parallel phases on rayon's global pool, and the work
//! runs on the pool's workers, not on the job thread that called it. So the
//! pool is built here once, before any job: one worker per core the kernel
//! has online (std::thread::available_parallelism, from MkProcStat), each
//! with a stack sized here, since a NONOS thread stack is a plain heap
//! allocation with no guard page beneath it. When that many workers cannot
//! be had, one; the proof is the same bytes either way, only slower.

use std::sync::atomic::{AtomicUsize, Ordering};

/// A worker's stack: four times the 2 MiB a rayon worker gets on Linux and
/// on the phones, where the same prover runs.
const STACK: usize = 8 << 20;

static WORKERS: AtomicUsize = AtomicUsize::new(0);

fn build(threads: usize) -> bool {
    rayon::ThreadPoolBuilder::new().num_threads(threads).stack_size(STACK).build_global().is_ok()
}

/// Build the pool: a worker per online core, or one.
pub fn start() {
    let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
    let built = if build(cores) {
        cores
    } else if cores > 1 && build(1) {
        1
    } else {
        0
    };
    WORKERS.store(built, Ordering::Release);
    crate::steps::serial(&format!("prover pool: {built} of {cores} cores"));
}

/// The workers the prover has, 0 when not even one could be started.
pub fn workers() -> usize {
    WORKERS.load(Ordering::Acquire)
}
