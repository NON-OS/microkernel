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

//! The per-CPU audit rings against the one shared ring they replace (copied
//! below from the code before): the same records, newest first, for any
//! interleaving of CPUs, and every record kept when CPUs push at once.

use std::sync::{Arc, Barrier};
use std::thread;

use super::percpu::{PerCpuLog, RING};

/// The shared ring as it was: 256 entries, newest overwrites oldest.
struct Shared {
    entries: [Option<u64>; 256],
    head: usize,
    count: usize,
}

impl Shared {
    fn new() -> Self {
        Self { entries: [None; 256], head: 0, count: 0 }
    }
    fn push(&mut self, e: u64) {
        self.entries[self.head] = Some(e);
        self.head = (self.head + 1) % 256;
        if self.count < 256 {
            self.count += 1;
        }
    }
    fn recent(&self, max: usize) -> Vec<u64> {
        let start = if self.count >= 256 { self.head } else { 0 };
        (0..self.count.min(max))
            .filter_map(|i| self.entries[(start + self.count - 1 - i) % 256])
            .collect()
    }
}

/// xorshift, so every run walks the same interleavings.
fn rng(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

#[test]
fn one_cpu_keeps_what_the_shared_ring_kept() {
    let log: PerCpuLog<u64, 4> = PerCpuLog::new();
    let mut shared = Shared::new();
    for e in 0..1000u64 {
        log.push(0, e);
        shared.push(e);
        assert_eq!(log.recent(256), shared.recent(256));
    }
}

#[test]
fn any_interleaving_reads_as_the_shared_ring() {
    let mut seed = 0x9e37_79b9_7f4a_7c15;
    for round in 0..50 {
        let log: PerCpuLog<u64, 8> = PerCpuLog::new();
        let mut shared = Shared::new();
        let cpus = 1 + (round % 8) as u64;
        for e in 0..(100 + 40 * round as u64) {
            log.push((rng(&mut seed) % cpus) as usize, e);
            shared.push(e);
        }
        for max in [0, 1, 17, 255, 256] {
            assert_eq!(log.recent(max), shared.recent(max), "round {round}, max {max}");
        }
    }
}

#[test]
fn each_cpu_keeps_its_last_ring_of_records_in_order() {
    let log: PerCpuLog<(usize, u64), 4> = PerCpuLog::new();
    for i in 0..600u64 {
        log.push(1, (1, i));
        if i % 3 == 0 {
            log.push(2, (2, i));
        }
    }
    let all = log.recent(usize::MAX);
    let one: Vec<u64> = all.iter().filter(|r| r.0 == 1).map(|r| r.1).collect();
    assert_eq!(one, (600 - RING as u64..600).rev().collect::<Vec<_>>());
    let two: Vec<u64> = all.iter().filter(|r| r.0 == 2).map(|r| r.1).collect();
    assert_eq!(two, (0..600).filter(|i| i % 3 == 0).rev().collect::<Vec<_>>());
}

#[test]
fn cpus_pushing_at_once_lose_nothing_and_keep_their_order() {
    const CPUS: usize = 8;
    const EACH: u64 = 200;
    for _ in 0..20 {
        let log: Arc<PerCpuLog<(usize, u64), CPUS>> = Arc::new(PerCpuLog::new());
        let start = Arc::new(Barrier::new(CPUS));
        let handles: Vec<_> = (0..CPUS)
            .map(|cpu| {
                let (log, start) = (log.clone(), start.clone());
                thread::spawn(move || {
                    start.wait();
                    for i in 0..EACH {
                        log.push(cpu, (cpu, i));
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().expect("pusher");
        }
        let all = log.recent(usize::MAX);
        assert_eq!(all.len(), CPUS * EACH as usize);
        for cpu in 0..CPUS {
            let mine: Vec<u64> = all.iter().filter(|r| r.0 == cpu).map(|r| r.1).collect();
            assert_eq!(mine, (0..EACH).rev().collect::<Vec<_>>(), "cpu {cpu}");
        }
    }
}

#[test]
fn two_writers_sharing_a_slot_lose_nothing() {
    let log: Arc<PerCpuLog<(usize, u64), 2>> = Arc::new(PerCpuLog::new());
    let handles: Vec<_> = (0..2)
        .map(|w| {
            let log = log.clone();
            thread::spawn(move || {
                for i in 0..100 {
                    log.push(0, (w, i));
                }
            })
        })
        .collect();
    for h in handles {
        h.join().expect("writer");
    }
    let all = log.recent(usize::MAX);
    assert_eq!(all.len(), 200);
    for w in 0..2 {
        let mine: Vec<u64> = all.iter().filter(|r| r.0 == w).map(|r| r.1).collect();
        assert_eq!(mine, (0..100).rev().collect::<Vec<_>>());
    }
}

#[test]
fn clear_empties_every_ring() {
    let log: PerCpuLog<u64, 4> = PerCpuLog::new();
    for e in 0..50 {
        log.push((e % 4) as usize, e);
    }
    log.clear();
    assert!(log.recent(usize::MAX).is_empty());
    log.push(3, 7);
    assert_eq!(log.recent(10), vec![7]);
}
