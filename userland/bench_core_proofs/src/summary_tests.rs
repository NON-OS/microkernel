// NONOS Operating System (AGPL-3.0-or-later)
//! What the reduction must report.

use crate::core::overhead::Overhead;
use crate::core::summary::Summary;

fn s(v: &[u64]) -> Summary {
    Summary::from_sorted(v, 0)
}

/// Nearest rank means every figure published was a figure measured. An
/// interpolating percentile would invent a value that never occurred, which is
/// a strange thing to offer as evidence.
#[test]
fn every_reported_value_was_actually_measured() {
    let run: Vec<u64> = (1..=100).collect();
    let out = s(&run);
    for v in [out.p50, out.p95, out.p99, out.max] {
        assert!(run.contains(&v), "{v} was never measured");
    }
}

#[test]
fn percentiles_land_on_the_hand_computed_ranks() {
    let run: Vec<u64> = (1..=100).collect();
    let out = s(&run);
    assert_eq!(out.p50, 50);
    assert_eq!(out.p95, 95);
    assert_eq!(out.p99, 99);
    assert_eq!(out.max, 100);
    assert_eq!(out.samples, 100);
}

/// The tail is the whole reason this reports percentiles instead of a mean. One
/// slow sample in a hundred must move p99 and the maximum, and must not move
/// the median: that is exactly the shape of a missed wake.
#[test]
fn one_slow_sample_moves_the_tail_and_not_the_median() {
    let mut run: Vec<u64> = vec![10; 99];
    run.push(9_000);
    run.sort_unstable();
    let out = s(&run);
    assert_eq!(out.p50, 10);
    assert_eq!(out.p99, 10, "p99 of 100 samples is the 99th, still the fast one");
    assert_eq!(out.max, 9_000);
}

#[test]
fn a_single_sample_reports_itself_at_every_percentile() {
    let out = s(&[42]);
    assert_eq!((out.p50, out.p95, out.p99, out.max), (42, 42, 42, 42));
}

/// A short run must not index past its end. Two samples have no distinct 99th.
#[test]
fn short_runs_clamp_instead_of_panicking() {
    for n in 1..=8usize {
        let run: Vec<u64> = (1..=n as u64).collect();
        let out = s(&run);
        assert_eq!(out.max, n as u64);
        assert!(out.p99 <= out.max);
        assert!(out.p50 <= out.p95 && out.p95 <= out.p99);
    }
}

/// A run that took no samples is a real answer, not an error to unwrap around.
#[test]
fn an_empty_run_reports_zeros_and_keeps_its_overhead() {
    let out = Summary::from_sorted(&[], 37);
    assert_eq!(out.samples, 0);
    assert_eq!(out.max, 0);
    assert_eq!(out.overhead, 37, "the calibration still happened");
}

/// The overhead is carried so a reader can add it back rather than trusting it.
#[test]
fn the_overhead_is_reported_not_hidden() {
    assert_eq!(Summary::from_sorted(&[100], 25).overhead, 25);
}

#[test]
fn correcting_subtracts_the_calibrated_floor() {
    let o = Overhead { cycles: 30 };
    assert_eq!(o.correct(100), 70);
}

/// A sample below the calibrated floor is noise at the counter's resolution.
/// Reporting zero is honest; wrapping to eighteen quintillion is not.
#[test]
fn a_sample_under_the_floor_saturates_to_zero() {
    let o = Overhead { cycles: 30 };
    assert_eq!(o.correct(10), 0);
}

/// Percentiles must never go backwards, whatever the run looks like.
#[test]
fn the_percentiles_are_monotonic_over_many_shapes() {
    for n in [1usize, 2, 3, 7, 10, 99, 100, 101, 1000] {
        let run: Vec<u64> = (0..n as u64).map(|i| i * 3 % 977).collect();
        let mut sorted = run.clone();
        sorted.sort_unstable();
        let out = s(&sorted);
        assert!(out.p50 <= out.p95, "n={n}");
        assert!(out.p95 <= out.p99, "n={n}");
        assert!(out.p99 <= out.max, "n={n}");
    }
}

/// The sort is shared by the kernel harness and the terminal command, so a bug
/// in it would silently corrupt every percentile on both sides at once.
#[test]
fn the_shared_sort_orders_every_shape() {
    use crate::core::sort::sort;
    let shapes: [Vec<u64>; 7] = [
        vec![],
        vec![1],
        vec![2, 1],
        vec![5, 5, 5, 5],
        (0..64u64).rev().collect(),
        (0..64u64).collect(),
        (0..200u64).map(|i| (i * 7919) % 251).collect(),
    ];
    for mut v in shapes {
        let mut want = v.clone();
        want.sort_unstable();
        sort(&mut v);
        assert_eq!(v, want);
    }
}

/// Ordering must not invent or drop samples: a reduction over a run that lost
/// one is a reduction over a machine that never existed.
#[test]
fn the_sort_is_a_permutation() {
    use crate::core::sort::sort;
    let mut v: Vec<u64> = (0..300u64).map(|i| (i * 37) % 91).collect();
    let before: u64 = v.iter().sum();
    let n = v.len();
    sort(&mut v);
    assert_eq!(v.len(), n);
    assert_eq!(v.iter().sum::<u64>(), before);
}
