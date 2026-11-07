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
//! Idle against active on the gateway link, for the shipped sender and the
//! Loopix one. A wire observer's test is the two-sample Kolmogorov-Smirnov
//! test on departure gaps, and on departures per second.

use crate::cover_model::{gaps, ks_two_sample, loopix, shipped, Kind, NYM_DEFAULT};

const HOUR_NS: u64 = 3_600_000_000_000;

struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

/// A user browsing: messages as a Poisson stream, `mean_ns` apart.
fn activity(rng: &mut SplitMix, mean_ns: u64) -> Vec<u64> {
    let mut at = 0u64;
    let mut out = Vec::new();
    loop {
        at += crate::exp_delay::exp_delay_ns(rng.next(), mean_ns).max(1);
        if at >= HOUR_NS {
            return out;
        }
        out.push(at);
    }
}

fn per_second(departures: &[(u64, Kind)]) -> Vec<u64> {
    let mut counts = vec![0u64; (HOUR_NS / 1_000_000_000) as usize];
    for &(t, _) in departures {
        counts[(t / 1_000_000_000) as usize] += 1;
    }
    counts
}

#[test]
fn the_shipped_sender_tells_idle_from_active() {
    let mut rng = SplitMix(1);
    let idle = shipped(&[], HOUR_NS);
    let active = shipped(&activity(&mut rng, 100_000_000), HOUR_NS);
    let (d, p) = ks_two_sample(&gaps(&idle), &gaps(&active));
    println!("shipped gaps   idle n={} active n={} D={d:.4} p={p:.3e}", idle.len(), active.len());
    assert!(p < 1e-6, "D={d} p={p}");
    let (d, p) = ks_two_sample(&per_second(&idle), &per_second(&active));
    println!("shipped per-s  D={d:.4} p={p:.3e}");
    assert!(p < 1e-6);
}

#[test]
fn the_loopix_sender_does_not() {
    let mut rng = SplitMix(2);
    let mut draw = || rng.next();
    let (idle, _) = loopix(&[], HOUR_NS, NYM_DEFAULT, &mut draw);
    let mut arrivals_rng = SplitMix(3);
    let arrivals = activity(&mut arrivals_rng, 100_000_000);
    let mut rng = SplitMix(4);
    let mut draw = || rng.next();
    let (active, waits) = loopix(&arrivals, HOUR_NS, NYM_DEFAULT, &mut draw);
    assert_eq!(waits.len(), arrivals.len(), "every real packet left within the hour");
    assert!(active.iter().any(|e| e.1 == Kind::Real) && idle.iter().all(|e| e.1 != Kind::Real));
    let (d, p) = ks_two_sample(&gaps(&idle), &gaps(&active));
    println!("loopix gaps    idle n={} active n={} D={d:.4} p={p:.3}", idle.len(), active.len());
    assert!(p > 0.05, "D={d} p={p}");
    let (d, p) = ks_two_sample(&per_second(&idle), &per_second(&active));
    println!("loopix per-s   D={d:.4} p={p:.3}");
    assert!(p > 0.05, "D={d} p={p}");
    let mean_wait = waits.iter().sum::<u64>() as f64 / waits.len() as f64 / 1e6;
    let mut sorted = waits.clone();
    sorted.sort_unstable();
    let p99 = sorted[sorted.len() * 99 / 100] as f64 / 1e6;
    println!("loopix queue wait for real packets: mean {mean_wait:.1} ms, p99 {p99:.1} ms, n={}", waits.len());
}

#[test]
fn the_loopix_result_holds_across_seeds_and_loads() {
    // Twenty pairs at three loads; at a 5% level about one pair in twenty
    // rejects by chance, so the bar is the count, not every pair.
    for mean_ms in [50u64, 100, 400] {
        let mut rejected = 0;
        for seed in 0..20u64 {
            let mut a = SplitMix(100 + seed);
            let mut draw = || a.next();
            let (idle, _) = loopix(&[], HOUR_NS / 6, NYM_DEFAULT, &mut draw);
            let mut r = SplitMix(200 + seed);
            let arrivals: Vec<u64> =
                activity(&mut r, mean_ms * 1_000_000).into_iter().filter(|&t| t < HOUR_NS / 6).collect();
            let mut b = SplitMix(300 + seed);
            let mut draw = || b.next();
            let (active, _) = loopix(&arrivals, HOUR_NS / 6, NYM_DEFAULT, &mut draw);
            if ks_two_sample(&gaps(&idle), &gaps(&active)).1 < 0.05 {
                rejected += 1;
            }
        }
        println!("loopix load 1 msg / {mean_ms} ms: {rejected} of 20 pairs rejected at 5%");
        assert!(rejected <= 4, "{rejected} of 20 at {mean_ms} ms");
    }
}

#[test]
fn ks_tells_two_different_exponentials_apart_and_one_from_itself() {
    let mut r = SplitMix(9);
    let a: Vec<u64> = (0..5000).map(|_| crate::exp_delay::exp_delay_ns(r.next(), 1000)).collect();
    let b: Vec<u64> = (0..5000).map(|_| crate::exp_delay::exp_delay_ns(r.next(), 1000)).collect();
    let c: Vec<u64> = (0..5000).map(|_| crate::exp_delay::exp_delay_ns(r.next(), 1200)).collect();
    assert!(ks_two_sample(&a, &b).1 > 0.01);
    assert!(ks_two_sample(&a, &c).1 < 1e-6);
}
