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

//! Proving many slots at once. Each proof is independent and peaks near 3 GB,
//! so the workers are bounded by cores and by memory, and the trailers come
//! back in slot order whatever order they finish in.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use nonos_attest_path::Kind;

use super::prove::prove_v4;

/// One slot to prove: its kind, its context and its v3 path.
pub struct Job {
    pub kind: Kind,
    pub ctx: Vec<u8>,
    pub path: Vec<u8>,
}

pub fn prove_all(root: &[u8; 32], jobs: &[Job]) -> Result<Vec<Vec<u8>>, String> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<Result<Vec<u8>, String>>>> = Mutex::new(vec![None; jobs.len()]);
    let done = AtomicUsize::new(0);
    let start = Instant::now();
    let n = workers().min(jobs.len());
    eprintln!("progress: proving {} slots, {n} at a time", jobs.len());
    std::thread::scope(|s| {
        for _ in 0..n {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(j) = jobs.get(i) else { break };
                let r = prove_v4(root, j.kind, &j.ctx, &j.path).map_err(|e| format!("slot {i}: {e}"));
                if let Ok(mut o) = out.lock() {
                    o[i] = Some(r);
                }
                progress(done.fetch_add(1, Ordering::Relaxed) + 1, jobs.len(), i, start);
            });
        }
    });
    let done = out.into_inner().map_err(|_| "a prover thread panicked")?;
    done.into_iter().enumerate().map(|(i, r)| r.unwrap_or_else(|| Err(format!("slot {i} was never proven")))).collect()
}

/*
 * One line per finished proof, on stderr, so a run of hours says how far it is.
 * It reads nothing the proofs read and changes no byte they write.
 */
fn progress(done: usize, total: usize, slot: usize, start: Instant) {
    let secs = start.elapsed().as_secs();
    let left = secs * (total - done) as u64 / done as u64;
    eprintln!(
        "progress: proved {done} of {total} (slot {slot}), {} elapsed, about {} left",
        clock(secs),
        clock(left)
    );
}

fn clock(secs: u64) -> String {
    if secs >= 3600 {
        format!("{}h{:02}m", secs / 3600, secs % 3600 / 60)
    } else {
        format!("{}m{:02}s", secs / 60, secs % 60)
    }
}

/*
 * NONOS_ENROLL_JOBS when set. Otherwise as many proofs at once as the machine
 * holds: one proof keeps about two cores busy and peaks near 3 GB, so half the
 * cores, no more than the memory allows at 3.5 GB each, at most 8.
 */
fn workers() -> usize {
    if let Some(n) = std::env::var("NONOS_ENROLL_JOBS").ok().and_then(|v| v.parse().ok()) {
        return usize::max(n, 1);
    }
    let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
    let by_cores = (cores / 2).clamp(1, 8);
    match memory_bytes() {
        Some(m) => by_cores.min((m / PER_PROOF) as usize).max(1),
        None => (cores / 4).clamp(1, 8),
    }
}

const PER_PROOF: u64 = 3_500_000_000;

/// The machine's memory: /proc/meminfo on Linux, hw.memsize on macOS.
fn memory_bytes() -> Option<u64> {
    if let Ok(info) = std::fs::read_to_string("/proc/meminfo") {
        let kb = info.lines().find(|l| l.starts_with("MemTotal:"))?.split_whitespace().nth(1)?;
        return kb.parse::<u64>().ok().map(|k| k * 1024);
    }
    let out = std::process::Command::new("sysctl").args(["-n", "hw.memsize"]).output().ok()?;
    String::from_utf8(out.stdout).ok()?.trim().parse().ok()
}
