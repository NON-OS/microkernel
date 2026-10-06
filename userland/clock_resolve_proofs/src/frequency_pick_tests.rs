// NONOS Operating System (AGPL-3.0-or-later)
//! The TSC rate an AP records. The PIT poll the last source stands for has
//! no timeout, so the proofs are that it runs only when nothing else answers.

use crate::frequency_pick::pick_tsc_hz;
use core::cell::Cell;

#[test]
fn a_settled_rate_is_taken_and_nothing_is_measured() {
    let asked = Cell::new(0);
    let hz = pick_tsc_hz(
        3_600_000_000,
        || {
            asked.set(asked.get() + 1);
            Some(1)
        },
        || {
            asked.set(asked.get() + 10);
            2
        },
    );
    assert_eq!(hz, 3_600_000_000);
    assert_eq!(asked.get(), 0);
}

#[test]
fn amd_with_a_settled_rate_never_reaches_the_pit() {
    // An AMD part: leaf 0x15 and 0x16 absent, the boot CPU measured against
    // the ACPI PM timer.
    let pit = Cell::new(false);
    let hz = pick_tsc_hz(
        2_994_000_000,
        || None,
        || {
            pit.set(true);
            0
        },
    );
    assert_eq!(hz, 2_994_000_000);
    assert!(!pit.get());
}

#[test]
fn before_the_boot_cpu_settles_cpuid_comes_before_the_pit() {
    let pit = Cell::new(false);
    let hz = pick_tsc_hz(
        0,
        || Some(2_400_000_000),
        || {
            pit.set(true);
            0
        },
    );
    assert_eq!(hz, 2_400_000_000);
    assert!(!pit.get());
}

#[test]
fn the_pit_is_the_last_resort() {
    assert_eq!(pick_tsc_hz(0, || None, || 1_800_000_000), 1_800_000_000);
}
