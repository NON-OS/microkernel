// NONOS Operating System (AGPL-3.0-or-later)
//! The bit clock never faster than a device asked for, and the four SPI
//! modes as polarity and phase.

use crate::core::{scr_for, sscr0, sscr1};
use crate::facts::{SSCR0_SCR_MAX, SSCR0_SSE, SSCR1_SPH, SSCR1_SPO};

#[test]
fn the_bit_clock_never_exceeds_what_was_asked() {
    for input in [100_000_000u32, 120_000_000, 133_000_000] {
        for want in [400_000u32, 1_000_000, 8_000_000, 25_000_000, 50_000_000, 200_000_000] {
            let scr = scr_for(input, want).expect("within the divider");
            let got = input / (scr + 1);
            assert!(got <= want, "{input} for {want}: {got}");
            assert!(scr == 0 || input / scr > want, "and no slower than need be");
        }
    }
}

#[test]
fn a_clock_slower_than_the_divider_reaches_is_refused() {
    // 100 MHz over 4096 is about 24 kHz: 1 kHz cannot be made, and running
    // faster than a device asked is never done.
    assert_eq!(scr_for(100_000_000, 1_000), None);
    assert_eq!(scr_for(100_000_000, 0), None);
    let slowest = 100_000_000u32.div_ceil(SSCR0_SCR_MAX + 1);
    assert_eq!(scr_for(100_000_000, slowest), Some(SSCR0_SCR_MAX));
    assert_eq!(scr_for(100_000_000, slowest - 1), None);
}

#[test]
fn frames_are_bytes_and_the_port_starts_disabled() {
    let v = sscr0(9);
    assert_eq!((v & 0xF, v >> 4 & 3, v & SSCR0_SSE, v >> 8 & 0xFFF), (7, 0, 0, 9));
}

#[test]
fn the_four_modes_set_polarity_and_phase() {
    assert_eq!(sscr1(0), 0);
    assert_eq!(sscr1(1), SSCR1_SPH);
    assert_eq!(sscr1(2), SSCR1_SPO);
    assert_eq!(sscr1(3), SSCR1_SPO | SSCR1_SPH);
}
