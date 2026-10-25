// NONOS Operating System (AGPL-3.0-or-later)
//! Divisors at the boards' own clocks, and the nearest-divisor and refusal
//! rules swept over a range of rates.

use crate::uart::divisor::{mini_uart, ns16550, pl011};

const BAUDS: [u32; 6] = [9600, 19_200, 38_400, 57_600, 115_200, 921_600];

#[test]
fn the_boards_figures() {
    assert_eq!(pl011(24_000_000, 115_200), Some((13, 1)));
    assert_eq!(pl011(48_000_000, 115_200), Some((26, 3)));
    assert_eq!(ns16550(1_843_200, 115_200), Some(1));
    assert_eq!(ns16550(1_843_200, 9600), Some(12));
    assert_eq!(mini_uart(250_000_000, 115_200), Some(270));
    assert_eq!(mini_uart(500_000_000, 115_200), Some(542));
}

#[test]
fn a_rate_no_divisor_makes_is_refused() {
    for baud in BAUDS {
        assert_eq!(pl011(16 * baud - 1, baud), None);
        assert_eq!(ns16550(8 * baud - 1, baud), None);
        assert_eq!(mini_uart(4 * baud - 1, baud), None);
    }
    assert_eq!((pl011(1, 0), ns16550(1, 0), mini_uart(1, 0)), (None, None, None));
    // Past the 16-bit registers at the other end.
    assert_eq!(pl011(u32::MAX, 1), None);
    assert_eq!(ns16550(u32::MAX, 1), None);
    assert_eq!(mini_uart(u32::MAX, 1), None);
}

#[test]
fn a_fraction_that_rounds_to_one_carries() {
    // 1407 / 128 is 10 and 127/128, which rounds up to a whole 11.
    assert_eq!(pl011(1407, 8), Some((11, 0)));
}

/// `2 * |divisor * step - clock| <= step`: the divisor, in units of the
/// register's finest step, is within half a step of the exact quotient.
fn nearest(divisor: u64, step: u64, clock: u32) -> bool {
    2 * (divisor * step).abs_diff(clock as u64) <= step
}

#[test]
fn every_divisor_is_the_nearest_one() {
    for clock in (1_000_000..=500_000_000).step_by(997_331) {
        for baud in BAUDS {
            let baud64 = baud as u64;
            // The PL011's divisor counts in 64ths: `clock / (16 * baud)` is
            // `n / 64` with `n` rounded, so `n` is nearest to `4 * clock / baud`.
            if let Some((i, f)) = pl011(clock, baud) {
                let n = (i * 64 + f) as u64;
                assert!(2 * (n * baud64).abs_diff(4 * clock as u64) <= baud64);
            }
            if let Some(d) = ns16550(clock, baud) {
                assert!(nearest(d as u64, 16 * baud64, clock));
            }
            if let Some(r) = mini_uart(clock, baud) {
                assert!(nearest(r as u64 + 1, 8 * baud64, clock));
            }
        }
    }
}
