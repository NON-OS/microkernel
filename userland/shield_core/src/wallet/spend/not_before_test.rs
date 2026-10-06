// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::arithmetic_side_effects)]

use super::{not_before, on_grid, GRID};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn a_time_is_rounded_down_to_the_grid_and_never_zero() {
    assert_eq!(on_grid(0), GRID);
    assert_eq!(on_grid(599), GRID);
    assert_eq!(on_grid(600), 600);
    assert_eq!(on_grid(1_790_000_599), 1_790_000_400);
}

/// The not-before limb is on the grid, never zero, and the grid point just passed.
#[test]
fn a_fresh_not_before_is_the_grid_point_just_passed() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("a clock").as_secs();
    let nb = not_before().expect("a time");
    assert_eq!(nb % GRID, 0);
    assert!(nb != 0 && nb <= now + 1 && now < nb + GRID + 1, "{nb} against {now}");
}
